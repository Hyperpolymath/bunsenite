# Bunsenite Project

## Project Overview

Bunsenite is a Nickel configuration file parser with multi-language FFI bindings. It provides a Rust core library with a Zig C ABI layer that enables bindings for Deno (JavaScript/TypeScript), Rescript, and WebAssembly for browser and universal use.

**Status**: v0.1.0 - Code complete and working
**Repository**: https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite
**License**: Dual MIT + Palimpsest

## Project Structure

```
bunsenite/
├── src/
│   ├── lib.rs              # Main library entry point
│   ├── loader.rs           # Nickel file loader (nickel-lang-core 0.9.1 API)
│   └── wasm.rs             # WebAssembly bindings
├── bindings/
│   ├── deno/               # Deno FFI bindings (.ts files)
│   ├── rescript/           # Rescript C FFI bindings
│   └── wasm/               # WASM build target
├── target/release/         # Build artifacts (6.5MB CLI, 6.1MB shared library)
├── Cargo.toml              # Rust dependencies
├── Justfile                # Build commands (use instead of shell scripts)
├── CLAUDE.md               # This file - AI assistant context
├── CHANGELOG.md            # Version history
├── PUBLISHING.md           # Publishing instructions
├── NEXT_STEPS.md           # Post-release roadmap
└── LICENSE                 # MIT + Palimpsest dual license
```

## Technology Stack

**Core:**
- Language: Rust (2021 edition)
- Parser: nickel-lang-core 0.9.1
- Serialization: serde, serde_json

**FFI Layer:**
- C ABI: Zig (provides stable interface isolating consumers from Rust ABI changes)

**Bindings:**
- Deno: TypeScript with Deno.dlopen for native FFI
- Rescript: Direct C FFI bindings
- WebAssembly: wasm-bindgen for browser/universal deployment

**Build Tools:**
- Build system: Cargo + Justfile
- WASM tooling: wasm-pack (optional)

**Testing:**
- Framework: Rust built-in test framework
- Coverage: 7 unit tests + 1 doc test (all passing)

## Development Setup

### Prerequisites

- Rust toolchain (2021 edition or later)
- Zig compiler (for C ABI layer)
- just command runner (`cargo install just`)
- Optional: wasm-pack for WebAssembly builds (`cargo install wasm-pack`)
- Optional: Deno runtime for testing Deno bindings

### Installation

```bash
# Clone the repository
git clone https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite.git
cd bunsenite

# Build all targets
just all

# Or build individually
cargo build --release        # Rust library and CLI
just wasm                    # WebAssembly bindings (requires wasm-pack)
```

### Running the Project

```bash
# Run CLI
cargo run --release

# Run tests
cargo test

# Build release binaries
just all
```

## Code Conventions

### Naming Conventions

- **Files**: snake_case (Rust convention: `loader.rs`, `wasm.rs`)
- **Variables**: snake_case
- **Functions**: snake_case
- **Types/Structs**: PascalCase
- **Constants**: SCREAMING_SNAKE_CASE

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Lint with: `cargo clippy`
- Use explicit error types, avoid unwrap() in library code
- Document public APIs with `///` doc comments
- Prefer explicit over implicit (no magic values)

### Testing Conventions

- Test files: Inline with `#[cfg(test)]` modules
- Test location: Same file as implementation or `tests/` directory
- Coverage: Currently 8 tests (7 unit + 1 doc test), all passing
- Run with: `cargo test`

## Architecture

### Design Patterns

**FFI Layer Pattern**: Stable C ABI via Zig isolates consumers from Rust ABI changes. This allows language bindings to remain stable across Rust compiler versions.

**Multi-Target Compilation**: Single Rust core with multiple compilation targets:
- Native shared library (via Zig C ABI)
- WebAssembly module (via wasm-bindgen)
- CLI binary

### Key Components

1. **src/loader.rs**: Nickel file parser using nickel-lang-core 0.9.1
   - `Program::new_from_source()` with trace parameter
   - `eval_full()` for evaluation
   - Manual error conversion via `serde_json::to_value()`

2. **src/wasm.rs**: WebAssembly bindings for browser deployment
   - ~95% native performance
   - Universal compatibility

3. **bindings/deno/**: Deno FFI bindings
   - NOT plain TypeScript - Deno-specific syntax required
   - Uses `Deno.dlopen` for native FFI calls to Zig layer

4. **bindings/rescript/**: Direct C FFI bindings
   - Calls through Zig C ABI layer

### Data Flow

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
              │   (loader.rs)   │
              │                 │
              │ nickel-lang-core│
              │     0.9.1       │
              └─────────────────┘
```

## Development Workflow

### Branch Strategy

- `main` - Production-ready code (current: v0.1.0 at commit 1c58f782)
- `feature/*` - Feature branches
- `bugfix/*` - Bug fix branches
- `claude/*` - AI assistant working branches

**GitLab Access**: HTTP push may be disabled; use SSH or GitLab API for pushing changes.

### Commit Messages

Follow conventional commits format:

```
type(scope): subject

body (optional)

footer (optional)
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### Merge Request Process

1. Create feature branch from `main`
2. Make changes with clear, atomic commits
3. Write/update tests (ensure all 8 tests pass)
4. Update documentation
5. Create GitLab merge request with description of changes
6. Address review feedback
7. Merge when approved

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Current status: 8 tests passing (7 unit + 1 doc test)
```

### Writing Tests

- Use `#[cfg(test)]` modules for unit tests
- Place integration tests in `tests/` directory
- Test both success and error paths
- Verify API compatibility with nickel-lang-core 0.9.1

## Building and Deployment

### Build Process

```bash
# Build all targets
just all

# Build Rust library and CLI only
cargo build --release

# Build WebAssembly (requires wasm-pack)
just wasm

# Output locations:
# - CLI: target/release/bunsenite (6.5MB)
# - Shared lib: target/release/libbunsenite.so (6.1MB)
```

### Deployment

**Published Artifacts:**
- GitLab release: Tag v0.1.0 created
- Binaries: Available in target/release/

**Next Publishing Steps** (see PUBLISHING.md):
1. Create GitLab release with binaries
2. Publish to crates.io: `cargo publish`
3. Optional: Publish to AUR (see PUBLISHING.md for instructions)

## Common Tasks

### Adding a New Feature

1. Create feature branch: `git checkout -b feature/feature-name`
2. Implement feature in Rust core (src/)
3. Update FFI bindings if needed (bindings/)
4. Write tests with `#[cfg(test)]`
5. Run `cargo test` to verify
6. Run `cargo fmt` and `cargo clippy`
7. Update CLAUDE.md if introducing new patterns
8. Create GitLab merge request

### Adding a New Language Binding

1. Create directory: `bindings/language-name/`
2. Implement FFI calls to Zig C ABI layer OR use WASM bindings
3. Add build target to Justfile if needed
4. Document in CLAUDE.md
5. Add example usage

### Debugging

**Rust Core:**
- Enable debug output: `RUST_LOG=debug cargo run`
- Use `dbg!()` macro for quick debugging
- Run specific test: `cargo test test_name`

**FFI Issues:**
- Check Zig C ABI compatibility
- Verify function signatures match between Rust and C
- Use `cargo build --verbose` for detailed build output

**nickel-lang-core Compatibility:**
- Refer to src/loader.rs for 0.9.1 API usage patterns
- Key changes: `Program::new_from_source()` requires trace parameter
- Manual error conversion, no `into_diagnostics()`

## Troubleshooting

### Common Issues

**Build Failures:**
- Ensure Rust toolchain is up to date: `rustup update`
- Check Zig is installed for FFI layer
- Run `cargo clean` and rebuild

**Test Failures:**
- Verify nickel-lang-core version is 0.9.1
- Check test files exist and are readable
- Run with `--nocapture` for debug output

**WASM Build Issues:**
- Install wasm-pack: `cargo install wasm-pack`
- Check wasm-bindgen version compatibility
- Clear target/wasm directory and rebuild

**API Compatibility:**
- If upgrading nickel-lang-core, review src/loader.rs
- Check for API changes in nickel-lang-core changelog
- Update trace parameters and error handling as needed

## Resources

### Documentation

- nickel-lang-core: https://github.com/tweag/nickel
- nickel-lang-core 0.9.1 docs: https://docs.rs/nickel-lang-core/0.9.1
- Zig FFI guide: https://ziglang.org/documentation/master/#C
- wasm-bindgen: https://rustwasm.github.io/docs/wasm-bindgen/

### Related Projects

- nickel-lang-core: Core Nickel language implementation
- Deno: Runtime for JavaScript/TypeScript with native FFI
- Rescript: Typed JavaScript alternative with C FFI support

### Internal Documentation

- CHANGELOG.md: Version history and release notes
- PUBLISHING.md: Instructions for publishing to crates.io and AUR
- NEXT_STEPS.md: Post-release roadmap and future plans

## Notes for AI Assistants

### Important Context

- **Project Status**: v0.1.0 code complete, all tests passing
- **Repository Location**: GitLab (NOT GitHub) at https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite
- **Working Directory**: /home/user/bunsenite (NOT zotero-voyant-export)
- **Current Commit**: Main branch at 1c58f782, tag v0.1.0 created

### Critical Design Decisions

**REQUIRED Technologies** (per user specifications):
- ✅ YES: Rust core
- ✅ YES: Zig C ABI layer (for stable FFI)
- ✅ YES: Deno bindings (TypeScript via Deno.dlopen - NOT plain TypeScript)
- ✅ YES: Rescript bindings (via C FFI)
- ✅ YES: WebAssembly bindings (for browser/universal use, ~95% native speed)
- ✅ YES: Justfile for builds
- ❌ NO: Plain TypeScript files (use Deno-specific .ts with Deno.dlopen)
- ❌ NO: Shell scripts (deleted build.sh per user request, use Justfile only)

**Architectural Rationale:**
- Zig FFI Layer: Provides stable C ABI, isolating consumers from Rust ABI changes
- WASM Addition: Enables browser deployment and universal compatibility
- Deno .ts Files: Required syntax for Deno runtime FFI, NOT plain TypeScript

### API Compatibility Gotchas

**nickel-lang-core 0.9.1 Breaking Changes** (see src/loader.rs):
1. `Program::new_from_source()` requires trace parameter: `std::io::sink()`
2. `eval_full()` takes no arguments (was previously different)
3. Manual error conversion required via `serde_json::to_value()`
4. NO `into_diagnostics()` method available (deprecated)

If code uses old API patterns, it WILL fail. Always refer to src/loader.rs for correct 0.9.1 usage.

### When Making Changes

- Follow the branch strategy (use `claude/*` branches)
- Maintain consistency with established patterns
- **Run cargo test - ensure all 8 tests pass**
- Update tests when modifying functionality
- Update this CLAUDE.md when introducing new conventions
- Use Justfile commands, NOT shell scripts
- Respect the technology choices listed above (don't suggest plain TypeScript, shell scripts, etc.)

### Code Quality Standards

- Write clean, readable code with clear intent
- Include comments for complex logic
- **Ensure all 8 tests pass before committing**
- Run `cargo fmt` and `cargo clippy`
- Avoid `unwrap()` in library code - use proper error handling
- Follow security best practices
- Verify FFI bindings maintain C ABI compatibility

### Build System

- **Primary**: Use `just all` for full builds
- **WASM**: Use `just wasm` (requires wasm-pack)
- **NO shell scripts**: User specifically removed build.sh, do not recreate
- **Test before commit**: Always run `cargo test`

### Suggested Improvements

When working on this project, consider:
- Performance optimizations for the Nickel parser
- Additional language bindings (following FFI or WASM pattern)
- Better error messages for end users
- Documentation improvements
- Example code for each binding type
- CI/CD integration for GitLab

## Project-Specific Guidelines

### FFI Development

When adding or modifying FFI bindings:
1. Maintain C ABI compatibility through Zig layer
2. Test across all target platforms
3. Document memory management requirements
4. Provide usage examples for each binding

### Dependency Management

- Keep nickel-lang-core at 0.9.1 (API compatibility critical)
- Avoid unnecessary dependencies
- Prefer std library over external crates when possible
- Document rationale for new dependencies in commit messages

### Testing Requirements

- All 8 tests must pass before any commit
- Add tests for new features
- Test both success and error paths
- Include FFI binding tests when applicable

### Documentation Standards

- Update CLAUDE.md when patterns change
- Keep CHANGELOG.md current
- Document breaking changes prominently
- Include code examples in API documentation

## Next Steps (Post v0.1.0)

See NEXT_STEPS.md for complete roadmap. Priority items:

1. **Publishing** (5-10 min each):
   - Create GitLab release with binaries
   - Publish to crates.io: `cargo publish`
   - Optional: Publish to AUR

2. **Enhancements**:
   - Improve error messages
   - Add more usage examples
   - Performance benchmarking
   - CI/CD pipeline for GitLab

3. **Community**:
   - Announce on /r/rust
   - Share on relevant forums
   - Gather feedback from users

## Changelog

Track major changes to project structure and conventions:

- **2025-11-21**: Initial CLAUDE.md created with v0.1.0 project context
  - Documented architecture: Rust core + Zig FFI + multi-language bindings
  - Captured API compatibility requirements for nickel-lang-core 0.9.1
  - Established build system (Justfile, no shell scripts)
  - Documented critical design decisions and user preferences

---

**Note**: This document should be updated as the project evolves. Keep it current to help AI assistants and new developers understand the project quickly.
