# SigmaOS-genoffice-tests

> **GenOffice Test Suite** — Integration and unit tests for the GenOffice pipeline.

Part of the [SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS) ecosystem.  
Parity / inspiration: **SigmaOS GenOffice**

## Quickstart

```bash
cargo test    # run deterministic AAA unit tests
cargo check   # verify compilation
```

## Component Role

This crate is a component of the SigmaOS GenOffice Test Suite suite:
- Fully real Rust implementation (no stubs, no placeholders)
- Fail-fast validation with typed domain errors
- Deterministic unit tests (Arrange-Act-Assert pattern)
- Single Responsibility, Modular, Loose Coupling

## API

```rust
use sigmaos_genoffice_tests::*;
// See src/lib.rs for the full public API surface
```

## Documentation

Full documentation: [SigmaOS GitHub Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)

## License

MIT
