# Biscuit WASM Shim

Rust project providing a pure WebAssembly shim on top of the Biscuit (biscuit-auth) library. This repository compiles Biscuit primitives to WebAssembly so they can be consumed from other languages/environments (for example, Go via WASM).

## At a Glance
- Crate: `biscuit-wasm-shim`
- Language: Rust (edition 2024)
- Primary target: `wasm32-unknown-unknown`
- Key dependencies: `biscuit-auth` (features: wasm, serde-error), `serde`, `serde_json`, `rand`, `getrandom` (custom)

The code exposes functions related to keys (public/private), keypair generation, creating and manipulating Biscuit tokens, and authorization. Exports are provided via WASM-friendly interfaces to be called from the host.

## Prerequisites
- Rust and Cargo installed
- WASM target: `rustup target add wasm32-unknown-unknown`

## Build
To build a debug WASM artifact:

```bash
cargo build --target wasm32-unknown-unknown
```

To build a release WASM artifact:

```bash
cargo build --release --target wasm32-unknown-unknown
```

The resulting `.wasm` artifacts will be in `target/wasm32-unknown-unknown/{debug,release}`.

## Code Structure
- `src/lib.rs`: entry point, utility macros, and RNG setup
- `src/wasm_export.rs`: WASM export macro(s)
- `src/crypto/`: key handling (keypair, private/public keys) and exports
- `src/builder/`: builders for Biscuit and Authorizer and their exports
- `src/token/`: types and functions related to tokens (Biscuit, Authorizer)

## Logging / Debugging
A host-side function `print(ptr, len)` is expected. The `print_wasm!` macro writes messages from the WASM module via that host function. Ensure `print` is implemented in your WASM runtime/host environment.

## License
Refer to the corresponding Biscuit project license, or set the appropriate license for this repository. In absence of an explicit statement, treat this repository as experimental/demo.

## Notes
- This repository focuses on exporting Biscuit primitives to WASM. It does not provide high-level bindings for a specific language; you will need to wire the exported functions on the host side.
- Use the `release` profile for better performance and smaller artifacts (panic = abort).
