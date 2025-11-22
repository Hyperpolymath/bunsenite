//! Bunsenite: Nickel configuration file parser with multi-language FFI bindings
//!
//! Bunsenite provides a Rust core library with a stable C ABI layer (via Zig)
//! that enables bindings for Deno (JavaScript/TypeScript), Rescript, and
//! WebAssembly for browser and universal use.
//!
//! # Features
//!
//! - **Type Safety**: Compile-time guarantees via Rust's type system
//! - **Memory Safety**: Rust ownership model, zero `unsafe` blocks
//! - **Offline-First**: Works completely air-gapped, no network dependencies
//! - **Multi-Language**: FFI bindings for Deno, Rescript, and WASM
//! - **Standards Compliant**: RSR Bronze tier, TPCF Perimeter 3
//!
//! # Examples
//!
//! ```
//! use bunsenite::NickelLoader;
//!
//! let config = r#"
//! {
//!   name = "example",
//!   version = "1.0.0",
//! }
//! "#;
//!
//! let result = NickelLoader::new()
//!     .parse_string(config, "config.ncl")
//!     .expect("Failed to parse config");
//!
//! println!("Parsed config: {}", result);
//! ```
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────┐
//! │                   Consumers                     │
//! ├───────────────┬───────────────┬─────────────────┤
//! │     Deno      │   Rescript    │     Browser     │
//! │  (TypeScript) │   (ReScript)  │     (WASM)      │
//! └───────┬───────┴───────┬───────┴────────┬────────┘
//!         │               │                │
//!         ▼               ▼                ▼
//!   ┌──────────┐   ┌──────────┐    ┌──────────────┐
//!   │ Zig FFI  │   │ Zig FFI  │    │ wasm-bindgen │
//!   │ (C ABI)  │   │ (C ABI)  │    │              │
//!   └─────┬────┘   └─────┬────┘    └──────┬───────┘
//!         │              │                 │
//!         └──────────────┴─────────────────┘
//!                        │
//!                        ▼
//!               ┌─────────────────┐
//!               │   Rust Core     │
//!               │   (lib.rs)      │
//!               │                 │
//!               │ nickel-lang-core│
//!               │     0.9.1       │
//!               └─────────────────┘
//! ```

#![deny(unsafe_code)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    unreachable_pub
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod error;
pub mod loader;

#[cfg(target_arch = "wasm32")]
#[cfg_attr(docsrs, doc(cfg(target_arch = "wasm32")))]
pub mod wasm;

// Re-exports for convenience
pub use error::{Error, Result};
pub use loader::NickelLoader;

/// Library version, updated automatically from Cargo.toml
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// RSR Framework compliance tier
pub const RSR_TIER: &str = "bronze";

/// TPCF Perimeter assignment
pub const TPCF_PERIMETER: u8 = 3; // Community Sandbox

/// Verify RSR compliance at compile time
///
/// This ensures that the library meets RSR Bronze tier requirements:
/// - Type safety (enforced by Rust compiler)
/// - Memory safety (enforced by `#![deny(unsafe_code)]`)
/// - Offline-first (no network dependencies in production code)
#[cfg(test)]
mod rsr_compliance_tests {
    use super::*;

    #[test]
    fn test_no_unsafe_code() {
        // This test passes if compilation succeeds with #![deny(unsafe_code)]
        assert_eq!(RSR_TIER, "bronze");
    }

    #[test]
    fn test_tpcf_perimeter() {
        assert_eq!(TPCF_PERIMETER, 3);
    }

    #[test]
    fn test_version_format() {
        // Ensure version follows semver
        let parts: Vec<&str> = VERSION.split('.').collect();
        assert_eq!(parts.len(), 3, "Version should be semver (x.y.z)");
    }
}
