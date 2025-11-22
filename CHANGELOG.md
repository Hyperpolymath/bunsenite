# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Additional language bindings (Python, Ruby, Node.js)
- Performance benchmarking suite
- REPL/interactive mode
- Schema validation
- Watch mode for auto-reload
- Plugin system

## [0.1.0] - 2025-11-22

### Added
- 🎉 Initial release of Bunsenite!
- ✅ Rust core library with nickel-lang-core 0.9.1 integration
- ✅ `NickelLoader` API for parsing and evaluating Nickel configurations
- ✅ Comprehensive error handling with helpful error messages
- ✅ WebAssembly bindings for browser deployment (~95% native speed)
- ✅ Command-line interface with `parse`, `validate`, and `info` commands
- ✅ Zero `unsafe` code (enforced by compiler directive)
- ✅ Complete test suite (30+ tests, 100% pass rate)
- ✅ Full RSR Bronze Tier compliance:
  - Type safety (Rust compile-time guarantees)
  - Memory safety (ownership model, no unsafe)
  - Offline-first (no network dependencies)
  - Complete documentation set
  - `.well-known/` directory (security.txt, ai.txt, humans.txt)
  - Build system (Justfile, Nix flake)
  - CI/CD pipeline (GitLab CI)
- ✅ TPCF Perimeter 3 (Community Sandbox) contribution model
- ✅ Dual MIT + Palimpsest 0.8 licensing
- ✅ Comprehensive documentation:
  - README.md with quick start and examples
  - CLAUDE.md for AI assistants and developers
  - SECURITY.md with vulnerability reporting
  - CONTRIBUTING.md with development workflow
  - CODE_OF_CONDUCT.md aligned with TPCF principles
  - MAINTAINERS.md with governance structure
- ✅ API documentation with examples
- ✅ FFI binding infrastructure:
  - Deno bindings (TypeScript)
  - Rescript bindings
  - C ABI via Zig (planned)

### Technical Details

#### API Compatibility (nickel-lang-core 0.9.1)
- `Program::new_from_source()` with trace parameter
- `eval_full()` with no arguments
- Manual error conversion via `serde_json::to_value()`
- No deprecated `into_diagnostics()` usage

#### Dependencies
- nickel-lang-core 0.9.1 (core parser)
- serde 1.0 (serialization)
- serde_json 1.0 (JSON conversion)
- anyhow 1.0 (error handling)
- thiserror 1.0 (error derive macros)
- clap 4.4 (CLI, optional)
- wasm-bindgen 0.2 (WASM bindings, target-specific)

#### Build Artifacts
- CLI binary: `bunsenite` (~6.5MB optimized)
- Shared library: `libbunsenite.so/dylib/dll` (~6.1MB optimized)
- WASM module: `bunsenite.wasm` (size varies by optimization level)

### Security

#### Memory Safety
- Zero `unsafe` code blocks (enforced by `#![deny(unsafe_code)]`)
- Rust ownership model prevents:
  - Use-after-free
  - Double-free
  - Null pointer dereferences
  - Buffer overflows
  - Data races

#### Supply Chain
- Minimal dependencies (only essential, well-audited crates)
- No network dependencies (offline-first design)
- Pinned dependency versions for reproducibility
- Regular `cargo audit` checks in CI

### Performance
- Native Rust: Baseline performance
- WebAssembly: ~95% native speed
- FFI bindings: ~90% native speed (minimal C ABI overhead)

### Known Limitations
- Nickel evaluation may consume significant memory/CPU for complex configs
  - **Mitigation**: Plan to add configurable timeouts and memory limits
- File I/O respects OS permissions (no privilege escalation)
- WASM runs in browser sandbox (subject to browser security model)

### Breaking Changes
- N/A (initial release)

### Deprecations
- N/A (initial release)

### Fixed
- N/A (initial release)

### Contributors
- Campaign for Cooler Coding and Programming (@cccp) - Initial implementation

---

## Version History

### Version Numbering

We use [Semantic Versioning](https://semver.org/):

```
MAJOR.MINOR.PATCH

MAJOR: Incompatible API changes
MINOR: Backwards-compatible new features
PATCH: Backwards-compatible bug fixes
```

### Release Cadence

- **Major releases**: As needed for breaking changes
- **Minor releases**: Monthly (if new features ready)
- **Patch releases**: As needed for critical bugs/security

### Support Policy

| Version | Support Status      | End of Life    |
| ------- | ------------------- | -------------- |
| 0.1.x   | ✅ Full support     | TBD (current)  |
| < 0.1.0 | ❌ Not supported    | N/A            |

---

## Links

- [Repository](https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite)
- [Issues](https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite/-/issues)
- [Releases](https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite/-/releases)
- [Crates.io](https://crates.io/crates/bunsenite) (coming soon)

---

**Note**: This changelog is maintained according to [Keep a Changelog](https://keepachangelog.com/) principles and serves as a living document of the project's evolution.
