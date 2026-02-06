use std::path::PathBuf;

use inactu_agent_kit::{AgentExecutionRequest, InactuExecutionAdapter};

fn main() -> Result<(), inactu_sdk::SdkError> {
    let adapter = InactuExecutionAdapter::default();

    let out = adapter.verify_execute_parse(AgentExecutionRequest {
        bundle: PathBuf::from("./bundle"),
        keys: PathBuf::from("./public-keys.json"),
        keys_digest: None,
        policy: PathBuf::from("./policy.json"),
        input: PathBuf::from("./input.json"),
        receipt: PathBuf::from("./receipt.json"),
        require_cosign: false,
        oci_ref: None,
        allow_experimental: false,
    })?;

    println!("verify: {}", out.verify_stdout.trim());
    println!("run: {}", out.execute_stdout.trim());
    println!("receipt schema: {}", out.receipt.raw["schema_version"]);
    Ok(())
}
