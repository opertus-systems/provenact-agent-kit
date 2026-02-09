# provenact-agent-kit

[![Status](https://img.shields.io/badge/stability-0.x--alpha-orange)](./ARCHITECTURE.md)

Thin agent-facing adapters on top of `inactu-sdk`.

This repository is intentionally not an agent framework.

## Description

`provenact-agent-kit` provides lightweight Rust adapters for verify/execute/receipt
execution flows powered by `inactu-sdk`.

## Tags

- `inactu`
- `agent`
- `sdk`
- `adapter`
- `supply-chain`

## Scope

In scope:
- execution adapters over `inactu-sdk`
- helper request/response types for verify + execute + receipt parse
- integration points for external orchestrators

Out of scope:
- planner loops
- long-lived memory
- scheduling
- autonomous tool-selection logic

## Example

```rust
use inactu_agent_kit::{AgentExecutionRequest, InactuExecutionAdapter};

let adapter = InactuExecutionAdapter::default();
let out = adapter.verify_execute_parse(AgentExecutionRequest {
    bundle: "./bundle".into(),
    keys: "./public-keys.json".into(),
    keys_digest: Some("sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".into()),
    policy: "./policy.json".into(),
    input: "./input.json".into(),
    receipt: "./receipt.json".into(),
    require_cosign: false,
    oci_ref: None,
    allow_experimental: false,
})?;

println!("{}", out.receipt.raw["artifact"]);
# Ok::<(), inactu_sdk::SdkError>(())
```

## CI

- `.github/workflows/ci.yml` runs format, tests, and example checks.
- `.github/workflows/conformance-smoke.yml` validates adapter behavior against a real
  `inactu` substrate checkout and real bundles in `inactu-skills`.

## Local Conformance Smoke

```bash
INACTU_VECTOR_ROOT=../inactu \
INACTU_SKILLS_ROOT=../inactu-skills \
INACTU_CLI_BIN=../inactu/target/debug/inactu-cli \
cargo test --test conformance_smoke -- --nocapture
```

If `INACTU_CLI_BIN` is not set, the test attempts to build `inactu-cli` from
`INACTU_VECTOR_ROOT` (or from sibling `../inactu`).
