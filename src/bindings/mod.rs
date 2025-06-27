/// Python bindings
#[cfg(feature = "python-bindings")]
pub mod python;
/// Universal bindings for all platforms
#[cfg(any(
    feature = "universal-bindings",
    all(target_arch = "wasm32", target_os = "unknown")
))]
pub mod universal;
/// wasm prover and verifier
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub mod wasm;
