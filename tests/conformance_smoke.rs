use std::path::{Path, PathBuf};
use std::process::Command;

use provenact_agent_kit::{AgentExecutionRequest, ProvenactExecutionAdapter};
use provenact_sdk::CliRunner;
use sha2::{Digest, Sha256};

#[test]
fn verify_run_parse_against_skills_repo_smoke() {
    let Ok(provenact_root) = discover_provenact_root() else {
        eprintln!("skipping smoke: no local provenact-cli/provenact checkout configured");
        return;
    };
    let Ok(skills_root) = discover_skills_root() else {
        eprintln!("skipping smoke: no local provenact-skills/provenact-skills checkout configured");
        return;
    };

    let cli_bin =
        discover_or_build_provenact_cli(&provenact_root).expect("discover/build provenact-cli");
    let adapter = ProvenactExecutionAdapter::with_runner(CliRunner::new(cli_bin));

    let fixture = skills_root.join("skills/echo.minimal/0.1.1");
    let bundle = fixture;
    let keys = bundle.join("public-keys.json");
    let keys_digest = sha256_file(&keys).expect("keys digest should compute");

    let temp = tempfile::tempdir().expect("temp dir");
    let policy = temp.path().join("policy.json");
    let input = temp.path().join("input.json");
    let receipt = temp.path().join("receipt.json");

    std::fs::write(
        &policy,
        r#"{
  "version": 1,
  "trusted_signers": ["alice.dev"],
  "capability_ceiling": {
    "exec": false,
    "time": false
  }
}"#,
    )
    .expect("write policy");
    std::fs::write(&input, br#"{"message":"smoke"}"#).expect("write input");

    let out = adapter
        .verify_execute_parse(AgentExecutionRequest {
            bundle,
            keys,
            keys_digest: Some(keys_digest),
            policy,
            input,
            receipt: receipt.clone(),
            require_cosign: false,
            oci_ref: None,
            allow_experimental: false,
        })
        .expect("verify+run+parse should pass");

    assert!(receipt.is_file(), "receipt file should be written");
    assert!(out.verify_stdout.contains("OK verify"));
    assert!(out.execute_stdout.contains("OK run"));
    assert!(out.receipt.raw["artifact"].is_string());
    assert!(out.receipt.raw["inputs_hash"].is_string());
    assert!(out.receipt.raw["outputs_hash"].is_string());
    assert!(out.receipt.raw["caps_used"].is_array());
}

fn discover_provenact_root() -> Result<PathBuf, String> {
    if let Ok(root) = std::env::var("PROVENACT_VECTOR_ROOT") {
        return Ok(PathBuf::from(root));
    }
    let fallback = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or_else(|| "workspace parent not found".to_string())?
        .join("provenact-cli");
    if fallback.is_dir() {
        return Ok(fallback);
    }
    let legacy = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or_else(|| "workspace parent not found".to_string())?
        .join("provenact");
    if legacy.is_dir() {
        Ok(legacy)
    } else {
        Err(
            "PROVENACT_VECTOR_ROOT not set and sibling ../provenact-cli (or legacy ../provenact) not found"
                .to_string(),
        )
    }
}

fn discover_skills_root() -> Result<PathBuf, String> {
    if let Ok(root) = std::env::var("PROVENACT_SKILLS_ROOT") {
        return Ok(PathBuf::from(root));
    }
    let fallback = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or_else(|| "workspace parent not found".to_string())?
        .join("provenact-skills");
    if fallback.is_dir() {
        return Ok(fallback);
    }
    Err("PROVENACT_SKILLS_ROOT not set and sibling ../provenact-skills not found".to_string())
}

fn discover_or_build_provenact_cli(root: &Path) -> Result<PathBuf, String> {
    if let Ok(cli) = std::env::var("PROVENACT_CLI_BIN") {
        return Ok(PathBuf::from(cli));
    }

    let candidate = root.join("target/debug/provenact-cli");
    if candidate.is_file() {
        return Ok(candidate);
    }

    let output = Command::new("cargo")
        .arg("build")
        .arg("-p")
        .arg("provenact-cli")
        .current_dir(root)
        .output()
        .map_err(|e| format!("failed to invoke cargo build for provenact-cli: {e}"))?;

    if !output.status.success() {
        return Err(format!(
            "building provenact-cli failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(candidate)
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let bytes =
        std::fs::read(path).map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    let digest = Sha256::digest(bytes);
    Ok(format!("sha256:{digest:x}"))
}
