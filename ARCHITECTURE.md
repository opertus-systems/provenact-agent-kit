# Architecture

## Boundary

`inactu-agent-kit` adapts execution primitives from `inactu-sdk`.
It does not decide what to do next.

## Design

- `InactuExecutionAdapter` provides one call:
  - verify bundle
  - execute verified bundle
  - parse receipt
- The adapter delegates all trust-critical checks to `inactu-sdk`/`inactu-cli`.

## Explicit Non-Goals

- no planning loop
- no memory store
- no scheduler
- no workflow state machine
