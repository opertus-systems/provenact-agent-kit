use std::path::PathBuf;

use inactu_sdk::{CliRunner, ExecuteRequest, InactuSdk, Receipt, SdkError, VerifyRequest};

pub type Result<T> = std::result::Result<T, SdkError>;

#[derive(Debug, Clone)]
pub struct AgentExecutionRequest {
    pub bundle: PathBuf,
    pub keys: PathBuf,
    pub keys_digest: Option<String>,
    pub policy: PathBuf,
    pub input: PathBuf,
    pub receipt: PathBuf,
    pub require_cosign: bool,
    pub oci_ref: Option<String>,
    pub allow_experimental: bool,
}

#[derive(Debug, Clone)]
pub struct AgentExecutionOutput {
    pub verify_stdout: String,
    pub execute_stdout: String,
    pub receipt: Receipt,
}

#[derive(Debug, Clone)]
pub struct InactuExecutionAdapter<R = CliRunner> {
    sdk: InactuSdk<R>,
}

impl Default for InactuExecutionAdapter<CliRunner> {
    fn default() -> Self {
        Self {
            sdk: InactuSdk::default(),
        }
    }
}

impl<R> InactuExecutionAdapter<R>
where
    R: inactu_sdk::CommandRunner,
{
    pub fn with_runner(runner: R) -> Self {
        Self {
            sdk: InactuSdk::with_runner(runner),
        }
    }

    pub fn verify_execute_parse(&self, req: AgentExecutionRequest) -> Result<AgentExecutionOutput> {
        let verify = self.sdk.verify_bundle(VerifyRequest {
            bundle: req.bundle.clone(),
            keys: req.keys.clone(),
            keys_digest: req.keys_digest.clone(),
            require_cosign: req.require_cosign,
            oci_ref: req.oci_ref.clone(),
            allow_experimental: req.allow_experimental,
        })?;

        let exec = self.sdk.execute_verified(ExecuteRequest {
            bundle: req.bundle,
            keys: req.keys,
            keys_digest: req.keys_digest,
            policy: req.policy,
            input: req.input,
            receipt: req.receipt,
            require_cosign: req.require_cosign,
            oci_ref: req.oci_ref,
            allow_experimental: req.allow_experimental,
        })?;

        let receipt = self.sdk.parse_receipt(exec.receipt_path)?;
        Ok(AgentExecutionOutput {
            verify_stdout: verify.stdout,
            execute_stdout: exec.stdout,
            receipt,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;

    #[derive(Default)]
    struct FakeRunner {
        calls: std::sync::Mutex<Vec<Vec<String>>>,
    }

    impl inactu_sdk::CommandRunner for FakeRunner {
        fn run<I, S>(&self, args: I) -> inactu_sdk::Result<String>
        where
            I: IntoIterator<Item = S>,
            S: AsRef<OsStr>,
        {
            let collected = args
                .into_iter()
                .map(|a| a.as_ref().to_string_lossy().to_string())
                .collect::<Vec<_>>();
            let cmd = collected.first().cloned().unwrap_or_default();
            self.calls.lock().expect("lock").push(collected);
            match cmd.as_str() {
                "verify" => Ok("OK verify".to_string()),
                "run" => Ok("OK run".to_string()),
                _ => Ok("OK".to_string()),
            }
        }
    }

    #[test]
    fn adapter_executes_verify_then_run() {
        let runner = FakeRunner::default();
        let adapter = InactuExecutionAdapter::with_runner(runner);
        let dir = tempfile::tempdir().expect("tmp");
        let receipt_path = dir.path().join("receipt.json");
        std::fs::write(&receipt_path, r#"{"schema_version":"1.0.0"}"#).expect("write");

        let out = adapter
            .verify_execute_parse(AgentExecutionRequest {
                bundle: PathBuf::from("./bundle"),
                keys: PathBuf::from("./keys.json"),
                keys_digest: Some(
                    "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                        .to_string(),
                ),
                policy: PathBuf::from("./policy.json"),
                input: PathBuf::from("./input.json"),
                receipt: receipt_path,
                require_cosign: false,
                oci_ref: None,
                allow_experimental: false,
            })
            .expect("adapter ok");

        assert!(out.verify_stdout.contains("OK verify"));
        assert!(out.execute_stdout.contains("OK run"));
        assert_eq!(out.receipt.raw["schema_version"], "1.0.0");
    }
}
