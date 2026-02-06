use std::path::{Path, PathBuf};
use std::process::Command;

use inactu_agent_kit::{AgentExecutionRequest, InactuExecutionAdapter};
use inactu_sdk::CliRunner;

#[test]
fn verify_run_parse_against_skills_repo_smoke() {
    let Ok(inactu_root) = discover_inactu_root() else {
        eprintln!("skipping smoke: no local inactu checkout configured");
        return;
    };
    let Ok(skills_root) = discover_skills_root() else {
        eprintln!("skipping smoke: no local inactu-skills checkout configured");
        return;
    };

    let cli_bin = discover_or_build_inactu_cli(&inactu_root).expect("discover/build inactu-cli");
    let adapter = InactuExecutionAdapter::with_runner(CliRunner::new(cli_bin));

    let fixture = skills_root.join("skills/echo.minimal/0.1.1");
    let bundle = fixture;
    let keys = bundle.join("public-keys.json");

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
            keys_digest: None,
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

fn discover_inactu_root() -> Result<PathBuf, String> {
    if let Ok(root) = std::env::var("INACTU_VECTOR_ROOT") {
        return Ok(PathBuf::from(root));
    }
    let fallback = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or_else(|| "workspace parent not found".to_string())?
        .join("inactu");
    if fallback.is_dir() {
        Ok(fallback)
    } else {
        Err("INACTU_VECTOR_ROOT not set and sibling ../inactu not found".to_string())
    }
}

fn discover_skills_root() -> Result<PathBuf, String> {
    if let Ok(root) = std::env::var("INACTU_SKILLS_ROOT") {
        return Ok(PathBuf::from(root));
    }
    let fallback = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or_else(|| "workspace parent not found".to_string())?
        .join("inactu-skills");
    if fallback.is_dir() {
        Ok(fallback)
    } else {
        Err("INACTU_SKILLS_ROOT not set and sibling ../inactu-skills not found".to_string())
    }
}

fn discover_or_build_inactu_cli(root: &Path) -> Result<PathBuf, String> {
    if let Ok(cli) = std::env::var("INACTU_CLI_BIN") {
        return Ok(PathBuf::from(cli));
    }

    let candidate = root.join("target/debug/inactu-cli");
    if candidate.is_file() {
        return Ok(candidate);
    }

    let output = Command::new("cargo")
        .arg("build")
        .arg("-p")
        .arg("inactu-cli")
        .current_dir(root)
        .output()
        .map_err(|e| format!("failed to invoke cargo build for inactu-cli: {e}"))?;

    if !output.status.success() {
        return Err(format!(
            "building inactu-cli failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(candidate)
}
