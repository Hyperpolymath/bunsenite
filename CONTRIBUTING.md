# Contributing to Bunsenite

Thank you for your interest in contributing to Bunsenite! We welcome contributions from everyone.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)
- [Commit Messages](#commit-messages)
- [Pull Request Process](#pull-request-process)
- [TPCF Contribution Model](#tpcf-contribution-model)

## Code of Conduct

This project adheres to a Code of Conduct that all contributors are expected to follow. Please read [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md) before contributing.

**Summary**: Be respectful, inclusive, and constructive. We're all here to build great software together.

## How Can I Contribute?

### Reporting Bugs

- **Check existing issues** first to avoid duplicates
- **Use the bug report template** (if available)
- **Include**:
  - Clear description of the problem
  - Steps to reproduce
  - Expected vs. actual behavior
  - Environment (OS, Rust version, Bunsenite version)
  - Minimal reproducible example

### Suggesting Enhancements

- **Check existing issues** to see if already requested
- **Use the feature request template** (if available)
- **Describe**:
  - Use case and motivation
  - Proposed solution
  - Alternatives considered
  - Impact on existing users

### Code Contributions

All code contributions are welcome! See [Development Workflow](#development-workflow) below.

### Documentation

- Fix typos, clarify explanations
- Add examples and tutorials
- Improve API documentation
- Translate documentation (future)

## Development Setup

### Prerequisites

- **Rust 1.70+**: `rustup install stable`
- **just**: `cargo install just`
- **Git**: Version control
- **Optional**:
  - Zig compiler (for FFI layer)
  - `wasm-pack` (for WASM builds)
  - Deno runtime (for Deno bindings)

### Initial Setup

```bash
# 1. Fork the repository on GitLab

# 2. Clone your fork
git clone https://gitlab.com/YOUR-USERNAME/bunsenite.git
cd bunsenite

# 3. Add upstream remote
git remote add upstream https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite.git

# 4. Install development tools
cargo install just cargo-edit cargo-audit

# 5. Build the project
just all

# 6. Run tests
cargo test
```

## Development Workflow

### 1. Create a Branch

```bash
# Sync with upstream
git fetch upstream
git checkout main
git merge upstream/main

# Create feature branch
git checkout -b feature/your-feature-name
# or
git checkout -b bugfix/issue-number-description
```

### 2. Make Changes

- **Write code**: Follow [Coding Standards](#coding-standards)
- **Add tests**: Ensure tests pass
- **Update docs**: Keep documentation current
- **Run linters**: `cargo clippy && cargo fmt`

### 3. Test Thoroughly

```bash
# Run all tests
cargo test

# Run linter
cargo clippy -- -D warnings

# Format code
cargo fmt --check

# Security audit
cargo audit

# All-in-one check
just check
```

### 4. Commit Changes

See [Commit Messages](#commit-messages) for conventions.

```bash
git add .
git commit -m "feat: add new feature"
```

### 5. Push and Create MR

```bash
git push origin feature/your-feature-name
```

Then create a Merge Request (MR) on GitLab.

## Coding Standards

### Rust Style

Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/):

```rust
// ✅ Good
pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Value> {
    let source = std::fs::read_to_string(path)?;
    // ...
}

// ❌ Bad
pub fn parsefile(path: String) -> Value {
    let source = std::fs::read_to_string(path).unwrap(); // Don't unwrap in library code!
    // ...
}
```

### Key Principles

1. **No `unsafe` code**: Enforced by `#![deny(unsafe_code)]`
2. **No `unwrap()` in library code**: Use `Result` and `?` operator
3. **Descriptive names**: Functions and variables should explain themselves
4. **Comments for "why"**: Code shows "what", comments explain "why"
5. **Module-level docs**: Every module needs `//!` doc comments
6. **Public API docs**: Every public item needs `///` doc comments

### Naming Conventions

- **Files**: `snake_case.rs`
- **Types/Structs**: `PascalCase`
- **Functions/Variables**: `snake_case`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case`

### Error Handling

```rust
// ✅ Good
pub fn do_something() -> Result<Value> {
    let data = read_file()?;
    let parsed = parse_data(data)?;
    Ok(parsed)
}

// ❌ Bad
pub fn do_something() -> Value {
    let data = read_file().unwrap();
    let parsed = parse_data(data).unwrap();
    parsed
}
```

### Documentation

```rust
/// Parse a Nickel configuration from a string
///
/// This function takes Nickel source code and evaluates it to produce
/// a JSON value.
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
    // implementation
}
```

## Testing

### Test Requirements

- **All new features** must have tests
- **All bug fixes** must have regression tests
- **Public APIs** must have doc tests (examples in `///` comments)
- **Edge cases** should be tested
- **Aim for high coverage** (goal: >80%)

### Writing Tests

```rust
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
    fn test_parse_invalid_syntax() {
        let loader = NickelLoader::new();
        let source = r#"{ foo = }"#; // Invalid
        let result = loader.parse_string(source, "bad.ncl");
        assert!(result.is_err());
    }
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run with coverage (requires tarpaulin)
cargo tarpaulin --out Html
```

## Documentation

### Types of Documentation

1. **Code comments**: Explain complex logic
2. **API docs**: `///` for public items
3. **Module docs**: `//!` at top of files
4. **README.md**: Project overview
5. **CLAUDE.md**: Developer/AI assistant guide
6. **Examples**: `examples/` directory

### Documentation Standards

- **Complete sentences**: Start with capital, end with period
- **Examples**: Include working code examples
- **Links**: Link to related functions/types
- **Markdown**: Use markdown formatting in doc comments

### Building Docs

```bash
# Build and open docs
cargo doc --open

# Build docs with all features
cargo doc --all-features --no-deps
```

## Commit Messages

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style (formatting, whitespace)
- `refactor`: Code refactoring (no behavior change)
- `perf`: Performance improvement
- `test`: Adding or updating tests
- `chore`: Maintenance tasks (deps, build, CI)
- `ci`: CI/CD changes
- `build`: Build system changes

### Examples

```bash
# Simple feature
git commit -m "feat: add validation function for Nickel configs"

# Bug fix with issue reference
git commit -m "fix: handle empty file input correctly

Closes #42"

# Breaking change
git commit -m "feat!: change API to return Result instead of Option

BREAKING CHANGE: parse_string now returns Result<Value> instead of Option<Value>"

# Scoped commit
git commit -m "docs(readme): add installation instructions"
```

## Pull Request Process

### Before Submitting

- [ ] Code compiles without warnings
- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated (for significant changes)
- [ ] Commit messages follow conventions

### MR Description

Include:

- **Summary**: What does this MR do?
- **Motivation**: Why is this change needed?
- **Changes**: What changed?
- **Testing**: How was this tested?
- **Breaking changes**: Any breaking changes?
- **Closes**: Which issues does this close? (e.g., "Closes #42")

### Review Process

1. **Automated checks**: CI/CD must pass
2. **Code review**: At least one maintainer approval required
3. **Discussion**: Address review comments
4. **Approval**: Maintainer approves MR
5. **Merge**: Maintainer merges (or you can if you have permissions)

### After Merge

- Delete your feature branch (locally and remotely)
- Close related issues if not auto-closed
- Celebrate! 🎉

## TPCF Contribution Model

This project uses the **Tri-Perimeter Contribution Framework (TPCF)**:

### Perimeter 3: Community Sandbox (You Are Here!)

- **Open to all**: Anyone can contribute
- **Review required**: All contributions reviewed by maintainers
- **Trust building**: Consistent contributors may be invited to Perimeter 2
- **Reversibility**: All changes tracked in Git for easy reversal

### Contribution Path

```
Perimeter 3 (Community)
    ↓ (consistent quality contributions)
Perimeter 2 (Trusted Contributors)
    ↓ (deep expertise & commitment)
Perimeter 1 (Core Maintainers)
```

**Note**: All perimeters are equally valued. Perimeter 3 contributors are essential to the project's success!

## Getting Help

- **Chat**: [GitLab Discussions](https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite/-/issues)
- **Issues**: [File an issue](https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite/-/issues/new)
- **Email**: See [MAINTAINERS.md](./MAINTAINERS.md)

## License

By contributing, you agree that your contributions will be licensed under the dual MIT + Palimpsest License. See [LICENSE](./LICENSE).

## Recognition

Contributors are recognized in:

- CHANGELOG.md (for significant contributions)
- Git commit history (always)
- Release notes
- `.well-known/humans.txt`

---

**Thank you for contributing to Bunsenite!**

*Building politically autonomous, emotionally safe software together.*
