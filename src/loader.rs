//! Nickel file loader and parser
//!
//! This module provides the core functionality for loading and parsing Nickel
//! configuration files using nickel-lang-core 0.9.1.
//!
//! # API Compatibility Notes (nickel-lang-core 0.9.1)
//!
//! - `Program::new_from_source()` requires trace parameter: `std::io::sink()`
//! - `eval_full()` takes no arguments (changed in 0.9.1)
//! - Manual error conversion required via `serde_json::to_value()`
//! - NO `into_diagnostics()` method available (deprecated)

use crate::error::{Error, Result};
use nickel_lang_core::cache::InputFormat;
use nickel_lang_core::program::Program;
use serde_json::Value;
use std::io::Write;
use std::path::Path;

/// Nickel configuration loader
///
/// Provides methods to parse and evaluate Nickel configuration files.
///
/// # Examples
///
/// ```
/// use bunsenite::NickelLoader;
///
/// let loader = NickelLoader::new();
/// let config = r#"{ name = "example", version = "1.0.0" }"#;
/// let result = loader.parse_string(config, "test.ncl").unwrap();
/// ```
#[derive(Debug, Clone, Default)]
pub struct NickelLoader {
    /// Enable verbose error reporting
    verbose: bool,
}

impl NickelLoader {
    /// Create a new Nickel loader with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable verbose error reporting
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Parse and evaluate a Nickel configuration from a string
    ///
    /// # Arguments
    ///
    /// * `source` - The Nickel configuration source code
    /// * `name` - A name for this configuration (used in error messages)
    ///
    /// # Returns
    ///
    /// A JSON value representing the evaluated configuration
    ///
    /// # Errors
    ///
    /// Returns an error if parsing or evaluation fails
    ///
    /// # Examples
    ///
    /// ```
    /// use bunsenite::NickelLoader;
    ///
    /// let loader = NickelLoader::new();
    /// let result = loader.parse_string("{ foo = 42 }", "config.ncl");
    /// assert!(result.is_ok());
    /// ```
    pub fn parse_string(&self, source: &str, name: &str) -> Result<Value> {
        // Create a Program from source
        // API change in 0.9.1: new_from_source requires trace parameter
        let mut program = Program::new_from_source(
            source.to_string(),
            name.to_string(),
            InputFormat::Nickel,
            std::io::sink(), // Trace output (discarded)
        )
        .map_err(|e| {
            let msg = format!("{:?}", e);
            Error::parse_error(name, msg)
        })?;

        // Evaluate the program
        // API change in 0.9.1: eval_full takes no arguments
        let eval_result = program.eval_full().map_err(|e| {
            let msg = format!("{:?}", e);
            Error::evaluation_error(name, msg)
        })?;

        // Convert to JSON
        // API change in 0.9.1: Manual conversion required, no into_diagnostics()
        let json_value = serde_json::to_value(&eval_result).map_err(|e| {
            Error::serialization_error(format!("Failed to convert to JSON: {}", e))
        })?;

        Ok(json_value)
    }

    /// Parse and evaluate a Nickel configuration from a file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the Nickel configuration file
    ///
    /// # Returns
    ///
    /// A JSON value representing the evaluated configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or if parsing/evaluation fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use bunsenite::NickelLoader;
    ///
    /// let loader = NickelLoader::new();
    /// let result = loader.parse_file("config.ncl");
    /// ```
    pub fn parse_file<P: AsRef<Path>>(&self, path: P) -> Result<Value> {
        let path = path.as_ref();
        let source = std::fs::read_to_string(path)?;
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown.ncl");

        self.parse_string(&source, name)
    }

    /// Validate a Nickel configuration without evaluating it
    ///
    /// This performs parsing and type-checking but does not evaluate the program.
    ///
    /// # Arguments
    ///
    /// * `source` - The Nickel configuration source code
    /// * `name` - A name for this configuration (used in error messages)
    ///
    /// # Returns
    ///
    /// Ok(()) if the configuration is valid, Err otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use bunsenite::NickelLoader;
    ///
    /// let loader = NickelLoader::new();
    /// assert!(loader.validate("{ foo = 42 }", "test.ncl").is_ok());
    /// assert!(loader.validate("{ foo = }", "bad.ncl").is_err());
    /// ```
    pub fn validate(&self, source: &str, name: &str) -> Result<()> {
        // Just try to create a Program - this performs parsing and type-checking
        Program::new_from_source(
            source.to_string(),
            name.to_string(),
            InputFormat::Nickel,
            std::io::sink(),
        )
        .map_err(|e| {
            let msg = format!("{:?}", e);
            Error::parse_error(name, msg)
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_simple_record() {
        let loader = NickelLoader::new();
        let source = r#"{ name = "test", version = "1.0.0" }"#;
        let result = loader.parse_string(source, "test.ncl");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_with_computation() {
        let loader = NickelLoader::new();
        let source = r#"{ sum = 1 + 2 + 3, product = 4 * 5 }"#;
        let result = loader.parse_string(source, "math.ncl").unwrap();

        assert_eq!(result["sum"], 6);
        assert_eq!(result["product"], 20);
    }

    #[test]
    fn test_parse_with_strings() {
        let loader = NickelLoader::new();
        let source = r#"{ greeting = "Hello, " ++ "World!" }"#;
        let result = loader.parse_string(source, "strings.ncl").unwrap();

        assert_eq!(result["greeting"], "Hello, World!");
    }

    #[test]
    fn test_parse_invalid_syntax() {
        let loader = NickelLoader::new();
        let source = r#"{ foo = }"#; // Invalid: missing value
        let result = loader.parse_string(source, "bad.ncl");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_valid_config() {
        let loader = NickelLoader::new();
        let source = r#"{ foo = 42, bar = "baz" }"#;
        assert!(loader.validate(source, "test.ncl").is_ok());
    }

    #[test]
    fn test_validate_invalid_config() {
        let loader = NickelLoader::new();
        let source = r#"{ foo = }"#; // Invalid
        assert!(loader.validate(source, "bad.ncl").is_err());
    }

    #[test]
    fn test_verbose_mode() {
        let loader = NickelLoader::new().with_verbose(true);
        assert_eq!(loader.verbose, true);
    }

    #[test]
    fn test_default_constructor() {
        let loader = NickelLoader::default();
        assert_eq!(loader.verbose, false);
    }

    #[test]
    fn test_error_contains_filename() {
        let loader = NickelLoader::new();
        let source = r#"{ invalid syntax }"#;
        let result = loader.parse_string(source, "myconfig.ncl");

        match result {
            Err(e) => {
                let msg = format!("{}", e);
                assert!(msg.contains("myconfig.ncl"));
            }
            Ok(_) => panic!("Expected error"),
        }
    }
}
