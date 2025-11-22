//! Bunsenite CLI
//!
//! Command-line interface for parsing and evaluating Nickel configuration files

use bunsenite::{NickelLoader, VERSION};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[command(
    name = "bunsenite",
    version = VERSION,
    about = "Nickel configuration file parser with multi-language FFI bindings",
    long_about = "Bunsenite is a Nickel configuration file parser with multi-language FFI bindings.\n\
                  It provides a Rust core library with a stable C ABI layer (via Zig) that enables\n\
                  bindings for Deno (JavaScript/TypeScript), Rescript, and WebAssembly.\n\n\
                  RSR Compliance: Bronze Tier | TPCF Perimeter: 3 (Community Sandbox)"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse and evaluate a Nickel configuration file
    Parse {
        /// Path to the Nickel configuration file
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Pretty-print the output JSON
        #[arg(short, long)]
        pretty: bool,
    },

    /// Validate a Nickel configuration without evaluating it
    Validate {
        /// Path to the Nickel configuration file
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },

    /// Show version and compliance information
    Info,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Parse { file, pretty }) => {
            handle_parse(file, pretty, cli.verbose)
        }
        Some(Commands::Validate { file }) => {
            handle_validate(file, cli.verbose)
        }
        Some(Commands::Info) => {
            handle_info();
            Ok(())
        }
        None => {
            // No command specified, show help
            println!("{}", get_help_text());
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        if let Some(suggestion) = e.suggestion() {
            eprintln!("\nSuggestion: {}", suggestion);
        }
        process::exit(1);
    }
}

fn handle_parse(file: PathBuf, pretty: bool, verbose: bool) -> bunsenite::Result<()> {
    if verbose {
        eprintln!("Parsing file: {}", file.display());
    }

    let loader = NickelLoader::new().with_verbose(verbose);
    let result = loader.parse_file(&file)?;

    if pretty {
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    } else {
        println!("{}", serde_json::to_string(&result).unwrap());
    }

    if verbose {
        eprintln!("✓ Successfully parsed and evaluated");
    }

    Ok(())
}

fn handle_validate(file: PathBuf, verbose: bool) -> bunsenite::Result<()> {
    if verbose {
        eprintln!("Validating file: {}", file.display());
    }

    let source = std::fs::read_to_string(&file)?;
    let name = file
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown.ncl");

    let loader = NickelLoader::new().with_verbose(verbose);
    loader.validate(&source, name)?;

    println!("✓ Configuration is valid");

    Ok(())
}

fn handle_info() {
    println!("Bunsenite v{}", VERSION);
    println!();
    println!("A Nickel configuration file parser with multi-language FFI bindings");
    println!();
    println!("Features:");
    println!("  • Type Safety: Compile-time guarantees via Rust's type system");
    println!("  • Memory Safety: Rust ownership model, zero unsafe blocks");
    println!("  • Offline-First: Works completely air-gapped, no network dependencies");
    println!("  • Multi-Language: FFI bindings for Deno, Rescript, and WASM");
    println!();
    println!("Standards Compliance:");
    println!("  • RSR Framework: Bronze Tier");
    println!("  • TPCF Perimeter: 3 (Community Sandbox)");
    println!("  • License: Dual MIT + Palimpsest 0.8");
    println!();
    println!("Repository: https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite");
    println!();
}

fn get_help_text() -> String {
    format!(
        r#"Bunsenite v{VERSION}
Nickel configuration file parser

USAGE:
    bunsenite <COMMAND>

COMMANDS:
    parse       Parse and evaluate a Nickel configuration file
    validate    Validate a Nickel configuration without evaluating it
    info        Show version and compliance information
    help        Print this message or the help of the given subcommand(s)

OPTIONS:
    -v, --verbose    Enable verbose output
    -h, --help       Print help information
    -V, --version    Print version information

EXAMPLES:
    # Parse and evaluate a config file
    bunsenite parse config.ncl

    # Parse with pretty-printed output
    bunsenite parse config.ncl --pretty

    # Validate without evaluating
    bunsenite validate config.ncl

    # Show info
    bunsenite info

For more information, visit:
https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite
"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_info_runs() {
        // Just verify info command doesn't panic
        handle_info();
    }

    #[test]
    fn test_help_text_contains_version() {
        let help = get_help_text();
        assert!(help.contains(VERSION));
    }
}
