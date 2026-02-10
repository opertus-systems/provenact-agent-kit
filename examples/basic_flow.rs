use std::path::PathBuf;

use provenact_agent_kit::{AgentExecutionRequest, ProvenactExecutionAdapter};

fn main() -> Result<(), provenact_sdk::SdkError> {
    let adapter = ProvenactExecutionAdapter::default();

    let out = adapter.verify_execute_parse(AgentExecutionRequest {
        bundle: PathBuf::from("./bundle"),
        keys: PathBuf::from("./public-keys.json"),
        keys_digest: Some("sha256:<public-keys-json-digest>".into()),
        policy: PathBuf::from("./policy.json"),
        input: PathBuf::from("./input.json"),
        receipt: PathBuf::from("./receipt.json"),
        require_cosign: false,
        oci_ref: None,
        cosign_key: None,
        cosign_cert_identity: None,
        cosign_cert_oidc_issuer: None,
        allow_experimental: false,
    })?;

    println!("verify: {}", out.verify_stdout.trim());
    println!("run: {}", out.execute_stdout.trim());
    println!("receipt schema: {}", out.receipt.raw["schema_version"]);
    Ok(())
}
