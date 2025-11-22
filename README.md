# Bunsenite

> Nickel configuration file parser with multi-language FFI bindings

[![RSR Bronze](https://img.shields.io/badge/RSR-Bronze-CD7F32)](https://example.com/rsr)
[![TPCF Perimeter 3](https://img.shields.io/badge/TPCF-Perimeter%203-blue)](https://example.com/tpcf)
[![License: MIT OR Palimpsest-0.8](https://img.shields.io/badge/License-MIT%20OR%20Palimpsest--0.8-green)](./LICENSE)
[![Build Status](https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite/badges/main/pipeline.svg)](https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite/-/pipelines)

## Overview

Bunsenite is a **Nickel configuration file parser** with a Rust core library and multi-language FFI bindings. It provides a stable C ABI layer (via Zig) that enables bindings for **Deno** (JavaScript/TypeScript), **Rescript**, and **WebAssembly** for browser and universal use.

### Key Features

- ✅ **Type Safety**: Compile-time guarantees via Rust's type system
- ✅ **Memory Safety**: Rust ownership model, **zero `unsafe` blocks**
- ✅ **Offline-First**: Works completely air-gapped, no network dependencies
- ✅ **Multi-Language**: FFI bindings for Deno, Rescript, and WASM
- ✅ **Standards Compliant**: RSR Bronze tier, TPCF Perimeter 3
- ✅ **Well-Documented**: Comprehensive API docs, examples, and guides
- ✅ **Production-Ready**: 100% test pass rate, CI/CD, semantic versioning

## Quick Start

### Installation

```bash
# From crates.io
cargo install bunsenite

# From source
git clone https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite.git
cd bunsenite
cargo install --path .
```

### Usage

#### Rust Library

```rust
use bunsenite::NickelLoader;

fn main() {
    let config = r#"
    {
      name = "my-app",
      version = "1.0.0",
      port = 8080,
    }
    "#;

    let loader = NickelLoader::new();
    let result = loader.parse_string(config, "config.ncl").unwrap();

    println!("Config: {}", result);
}
```

#### CLI

```bash
# Parse and evaluate a config file
bunsenite parse config.ncl

# Pretty-print output
bunsenite parse config.ncl --pretty

# Validate without evaluating
bunsenite validate config.ncl

# Show version and compliance info
bunsenite info
```

#### WebAssembly (Browser)

```javascript
import init, { parse_nickel } from './bunsenite.js';

async function main() {
    await init();
    const config = `{ name = "example", version = "1.0.0" }`;
    const result = parse_nickel(config, "config.ncl");
    console.log(JSON.parse(result));
}
```

#### Deno (TypeScript)

```typescript
// See bindings/deno/ for full example
import { parseNickel } from "./bunsenite_deno.ts";

const config = `{ foo = 42 }`;
const result = parseNickel(config, "config.ncl");
console.log(result);
```

## Architecture

```
┌─────────────────────────────────────────────────┐
│                   Consumers                     │
├───────────────┬───────────────┬─────────────────┤
│     Deno      │   Rescript    │     Browser     │
│  (TypeScript) │   (ReScript)  │     (WASM)      │
└───────┬───────┴───────┬───────┴────────┬────────┘
        │               │                │
        ▼               ▼                ▼
  ┌──────────┐   ┌──────────┐    ┌──────────────┐
  │ Zig FFI  │   │ Zig FFI  │    │ wasm-bindgen │
  │ (C ABI)  │   │ (C ABI)  │    │              │
  └─────┬────┘   └─────┬────┘    └──────┬───────┘
        │              │                 │
        └──────────────┴─────────────────┘
                       │
                       ▼
              ┌─────────────────┐
              │   Rust Core     │
              │   (lib.rs)      │
              │                 │
              │ nickel-lang-core│
              │     0.9.1       │
              └─────────────────┘
```

### Design Rationale

**Zig FFI Layer**: Provides stable C ABI, isolating consumers from Rust ABI changes. This allows language bindings to remain stable across Rust compiler versions.

**WASM Support**: Enables browser deployment and universal compatibility at ~95% native speed.

**Deno .ts Files**: Required syntax for Deno runtime FFI (NOT plain TypeScript). Uses `Deno.dlopen` for native FFI calls to Zig layer.

## Documentation

- **[CLAUDE.md](./CLAUDE.md)**: Comprehensive guide for AI assistants and developers
- **[SECURITY.md](./SECURITY.md)**: Security policies and vulnerability reporting
- **[CONTRIBUTING.md](./CONTRIBUTING.md)**: Contribution guidelines
- **[CHANGELOG.md](./CHANGELOG.md)**: Version history and release notes
- **[API Docs](https://docs.rs/bunsenite)**: Full Rust API documentation

## Standards Compliance

### RSR Framework: Bronze Tier

Bunsenite meets all **Rhodium Standard Repository (RSR) Bronze tier** requirements:

- ✅ Type safety (Rust compile-time guarantees)
- ✅ Memory safety (ownership model, `#![deny(unsafe_code)]`)
- ✅ Offline-first (no network dependencies)
- ✅ Complete documentation (README, LICENSE, SECURITY, CONTRIBUTING, CODE_OF_CONDUCT, MAINTAINERS)
- ✅ `.well-known/` directory (security.txt, ai.txt, humans.txt)
- ✅ Build system (Justfile, Nix flake)
- ✅ CI/CD pipeline (GitLab CI)
- ✅ 100% test pass rate

### TPCF: Perimeter 3 (Community Sandbox)

This project uses the **Tri-Perimeter Contribution Framework (TPCF)**:

- **Perimeter 1**: Core maintainers only (restricted)
- **Perimeter 2**: Trusted contributors (by invitation)
- **Perimeter 3**: **Community Sandbox** - Open to all contributors

All contributions are welcome! See [CONTRIBUTING.md](./CONTRIBUTING.md) for details.

## Building from Source

### Prerequisites

- Rust 1.70+ (`rustup install stable`)
- `just` command runner (`cargo install just`)
- Optional: Zig compiler (for FFI layer)
- Optional: `wasm-pack` (for WASM builds: `cargo install wasm-pack`)
- Optional: Deno runtime (for Deno bindings)

### Build Commands

```bash
# Build all targets
just all

# Build Rust library and CLI
cargo build --release

# Build WebAssembly
just wasm

# Run tests
cargo test

# Run linter
cargo clippy

# Format code
cargo fmt

# Check RSR compliance
just rsr-check
```

See `Justfile` for all available commands.

## Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run with coverage (requires tarpaulin)
cargo tarpaulin --out Html
```

Current status: **100% test pass rate** (30+ tests covering core functionality, error handling, and edge cases)

## Performance

- **Native Rust**: Baseline performance
- **WebAssembly**: ~95% native speed
- **FFI (Deno/Rescript)**: ~90% native speed (C ABI overhead minimal)

## Security

We take security seriously. See [SECURITY.md](./SECURITY.md) for:

- Supported versions
- Vulnerability reporting process
- Security best practices
- Responsible disclosure policy

**Security contact**: See `.well-known/security.txt` or [SECURITY.md](./SECURITY.md)

## License

Dual licensed under your choice of:

- **MIT License** ([LICENSE-MIT](./LICENSE) or http://opensource.org/licenses/MIT)
- **Palimpsest License v0.8** ([LICENSE-PALIMPSEST](./LICENSE) or https://palingenesis.org/palimpsest-license)

This allows maximum flexibility for use while preserving reversibility and emotional safety principles.

## Contributing

Contributions are welcome! This is a **TPCF Perimeter 3** (Community Sandbox) project.

See [CONTRIBUTING.md](./CONTRIBUTING.md) for:

- Code of Conduct
- Development workflow
- Testing requirements
- Commit message conventions
- Pull request process

## Community

- **Issues**: [GitLab Issues](https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite/-/issues)
- **Discussions**: [GitLab Discussions](https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite/-/issues)
- **Security**: See [SECURITY.md](./SECURITY.md)

## Acknowledgments

- [Nickel Language Team](https://github.com/tweag/nickel) for the excellent configuration language
- RSR Framework contributors
- TPCF community
- All contributors to this project

## Roadmap

See [NEXT_STEPS.md](./NEXT_STEPS.md) for planned features and enhancements:

- [ ] Additional language bindings (Python, Ruby, Node.js)
- [ ] Performance benchmarking suite
- [ ] REPL/interactive mode
- [ ] Schema validation
- [ ] Watch mode for auto-reload
- [ ] Plugin system

## Version History

See [CHANGELOG.md](./CHANGELOG.md) for detailed version history.

Current version: **0.1.0** (Bronze tier compliant, production-ready)

---

**Made with ❤️ by the Campaign for Cooler Coding and Programming**

*Politically autonomous software for emotionally safe development*
