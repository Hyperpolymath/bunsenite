//! WebAssembly bindings for Bunsenite
//!
//! This module provides WASM bindings that enable Bunsenite to run in browsers
//! and other WASM environments with ~95% native performance.
//!
//! # Examples
//!
//! ```javascript
//! import init, { parse_nickel } from './bunsenite.js';
//!
//! async function main() {
//!     await init();
//!     const config = `{ name = "example", version = "1.0.0" }`;
//!     const result = parse_nickel(config, "config.ncl");
//!     console.log(JSON.parse(result));
//! }
//! ```

use crate::{Error, NickelLoader};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Initialize WASM module
///
/// This should be called once before using any other WASM functions.
/// It sets up panic hooks for better error messages in the browser.
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Parse and evaluate a Nickel configuration string
///
/// # Arguments
///
/// * `source` - The Nickel configuration source code
/// * `name` - A name for this configuration (used in error messages)
///
/// # Returns
///
/// A JSON string representing the evaluated configuration, or an error message
///
/// # Examples
///
/// ```javascript
/// const result = parse_nickel('{ foo = 42 }', 'config.ncl');
/// const config = JSON.parse(result);
/// console.log(config.foo); // 42
/// ```
#[wasm_bindgen]
pub fn parse_nickel(source: &str, name: &str) -> Result<String, JsValue> {
    let loader = NickelLoader::new();

    let result = loader
        .parse_string(source, name)
        .map_err(|e| JsValue::from_str(&format!("{}", e)))?;

    serde_json::to_string(&result)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Validate a Nickel configuration without evaluating it
///
/// # Arguments
///
/// * `source` - The Nickel configuration source code
/// * `name` - A name for this configuration (used in error messages)
///
/// # Returns
///
/// Ok(()) if valid, Err with error message if invalid
///
/// # Examples
///
/// ```javascript
/// try {
///     validate_nickel('{ foo = 42 }', 'config.ncl');
///     console.log('Valid!');
/// } catch (e) {
///     console.error('Invalid:', e);
/// }
/// ```
#[wasm_bindgen]
pub fn validate_nickel(source: &str, name: &str) -> Result<(), JsValue> {
    let loader = NickelLoader::new();

    loader
        .validate(source, name)
        .map_err(|e| JsValue::from_str(&format!("{}", e)))
}

/// Get library version
#[wasm_bindgen]
pub fn version() -> String {
    crate::VERSION.to_string()
}

/// Get RSR compliance tier
#[wasm_bindgen]
pub fn rsr_tier() -> String {
    crate::RSR_TIER.to_string()
}

/// Get TPCF perimeter
#[wasm_bindgen]
pub fn tpcf_perimeter() -> u8 {
    crate::TPCF_PERIMETER
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_parse_simple() {
        let source = r#"{ name = "test" }"#;
        let result = parse_nickel(source, "test.ncl");
        assert!(result.is_ok());
    }

    #[test]
    fn test_wasm_validate_valid() {
        let source = r#"{ foo = 42 }"#;
        let result = validate_nickel(source, "test.ncl");
        assert!(result.is_ok());
    }

    #[test]
    fn test_wasm_validate_invalid() {
        let source = r#"{ foo = }"#;
        let result = validate_nickel(source, "bad.ncl");
        assert!(result.is_err());
    }

    #[test]
    fn test_wasm_version() {
        let v = version();
        assert!(!v.is_empty());
    }

    #[test]
    fn test_wasm_rsr_metadata() {
        assert_eq!(rsr_tier(), "bronze");
        assert_eq!(tpcf_perimeter(), 3);
    }
}
