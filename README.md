# provenact-agent-kit

[![Status](https://img.shields.io/badge/stability-0.x--alpha-orange)](./ARCHITECTURE.md)

Thin agent-facing adapters on top of `provenact-sdk`.

This repository is intentionally not an agent framework.

## Description

`provenact-agent-kit` provides lightweight Rust adapters for verify/execute/receipt
execution flows powered by `provenact-sdk`.

## Tags

- `provenact`
- `agent`
- `sdk`
- `adapter`
- `supply-chain`

## Scope

In scope:
- execution adapters over `provenact-sdk`
- helper request/response types for verify + execute + receipt parse
- integration points for external orchestrators

Out of scope:
- planner loops
- long-lived memory
- scheduling
- autonomous tool-selection logic

## Example

```rust
use provenact_agent_kit::{AgentExecutionRequest, ProvenactExecutionAdapter};

let adapter = ProvenactExecutionAdapter::default();
let out = adapter.verify_execute_parse(AgentExecutionRequest {
    bundle: "./bundle".into(),
    keys: "./public-keys.json".into(),
    keys_digest: Some("sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".into()),
    policy: "./policy.json".into(),
    input: "./input.json".into(),
    receipt: "./receipt.json".into(),
    require_cosign: false,
    oci_ref: None,
    cosign_key: None,
    cosign_cert_identity: None,
    cosign_cert_oidc_issuer: None,
    allow_experimental: false,
})?;

println!("{}", out.receipt.raw["artifact"]);
# Ok::<(), provenact_sdk::SdkError>(())
```

## CI

- `.github/workflows/ci.yml` runs format, tests, and example checks.
- `.github/workflows/conformance-smoke.yml` validates adapter behavior against a real
  `provenact` substrate checkout and real bundles in `provenact-skills`.

## Local Conformance Smoke

```bash
PROVENACT_VECTOR_ROOT=../provenact-cli \
PROVENACT_SKILLS_ROOT=../provenact-skills \
PROVENACT_CLI_BIN=../provenact-cli/target/debug/provenact-cli \
cargo test --test conformance_smoke -- --nocapture
```

If `PROVENACT_CLI_BIN` is not set, the test attempts to build `provenact-cli` from
`PROVENACT_VECTOR_ROOT` (or from sibling `../provenact-cli`).
