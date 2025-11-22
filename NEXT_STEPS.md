# Bunsenite: Next Steps

Post-v0.1.0 roadmap and future enhancements.

## Immediate Tasks (Week 1-2)

### Publishing

- [ ] **Create GitLab Release** (5 min)
  - Navigate to: https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite/-/releases/new
  - Use tag: `v0.1.0`
  - Copy release notes from CHANGELOG.md
  - Attach binaries from `target/release/`

- [ ] **Publish to crates.io** (10 min)
  - Verify account: `cargo login`
  - Dry run: `cargo publish --dry-run`
  - Publish: `cargo publish`
  - Verify: https://crates.io/crates/bunsenite

- [ ] **Build WASM Module** (5 min)
  - Install wasm-pack: `cargo install wasm-pack`
  - Build: `just wasm`
  - Test in browser
  - Publish to npm (optional)

- [ ] **Publish to AUR** (30 min, optional)
  - See `PUBLISHING.md` for detailed instructions
  - Create PKGBUILD
  - Test in clean chroot
  - Submit to AUR

### Announcement

- [ ] **Post to /r/rust** (10 min)
  - Title: "Bunsenite v0.1.0: Nickel configuration parser with multi-language FFI"
  - Include: Overview, features, quick start, link
  - Flair: "release"

- [ ] **Share on Social Media** (5 min each)
  - Twitter/X
  - Mastodon
  - Hacker News (Show HN)
  - Lobsters

## Short Term (Month 1)

### Quality Improvements

- [ ] **Expand Test Suite** (2-3 days)
  - Add integration tests in `tests/` directory
  - Test FFI boundaries (Deno, Rescript, WASM)
  - Add property-based tests (quickcheck)
  - Edge cases: unicode, large files, complex computations
  - Target: 80%+ code coverage

- [ ] **Performance Benchmarking** (1-2 days)
  - Set up criterion.rs benchmarks
  - Baseline measurements
  - Compare: Nickel CLI vs. Bunsenite
  - Identify bottlenecks
  - Document performance characteristics

- [ ] **Improve Error Messages** (2-3 days)
  - More context in error messages
  - Suggestions for common mistakes
  - Line/column numbers for parse errors
  - Pretty-print error output (miette crate?)
  - User-friendly, actionable errors

### Documentation

- [ ] **Complete API Documentation** (1-2 days)
  - Ensure every public item has `///` docs
  - Add more examples to docs
  - Create comprehensive guide in docs/
  - Add diagrams for architecture

- [ ] **Create Tutorial** (1 day)
  - Step-by-step guide for beginners
  - Real-world use cases
  - Common patterns
  - Troubleshooting guide

- [ ] **Video Demonstration** (1 day, optional)
  - Screen recording showing usage
  - Upload to YouTube
  - Embed in README

### Infrastructure

- [ ] **Set Up Continuous Benchmarking** (1 day)
  - Benchmark on every commit
  - Track performance over time
  - Alert on regressions

- [ ] **Security Audit** (External, Q2 2025)
  - Professional security review
  - Fuzzing with cargo-fuzz
  - Static analysis with cargo-audit, cargo-deny
  - Document findings

## Medium Term (Months 2-3)

### Features

- [ ] **Configuration Validation** (3-5 days)
  ```rust
  // Define schema
  let schema = Schema::new()
      .field("name", Type::String)
      .field("port", Type::Integer.range(1..=65535));

  // Validate against schema
  loader.validate_with_schema(config, schema)?;
  ```

- [ ] **Watch Mode** (2-3 days)
  ```bash
  bunsenite watch config.ncl --on-change "restart-server"
  ```
  - Auto-reload on file changes
  - Execute command on change
  - Debouncing support

- [ ] **REPL/Interactive Mode** (3-5 days)
  ```bash
  bunsenite repl
  > { foo = 42 }
  { foo = 42 }
  > foo + 8
  50
  ```
  - Interactive Nickel evaluation
  - Tab completion
  - History support
  - Syntax highlighting

- [ ] **Multiple File Support** (2-3 days)
  ```rust
  let loader = NickelLoader::new()
      .import("base.ncl")
      .import("overrides.ncl")
      .merge_strategy(MergeStrategy::Deep);
  ```
  - Import/merge multiple configs
  - Resolve conflicts
  - Deep merge support

- [ ] **JSON Schema Export** (2-3 days)
  ```bash
  bunsenite schema config.ncl > schema.json
  ```
  - Generate JSON Schema from Nickel types
  - Use for validation in other tools
  - IDE integration

### Additional Language Bindings

- [ ] **Python Bindings** (3-5 days)
  - PyO3 for native bindings
  - Type hints
  - Publish to PyPI
  - Examples and docs

- [ ] **Ruby Bindings** (3-5 days)
  - Rutie or Magnus for FFI
  - Idiomatic Ruby API
  - Publish to RubyGems
  - Examples and docs

- [ ] **Node.js Bindings** (2-3 days)
  - NAPI-RS for native bindings
  - TypeScript definitions
  - Publish to npm
  - Examples and docs

- [ ] **Go Bindings** (2-3 days)
  - CGo for C FFI
  - Idiomatic Go API
  - Examples and docs

### Tooling

- [ ] **VS Code Extension** (5-7 days)
  - Syntax highlighting for .ncl files
  - Validation on save
  - Inline error messages
  - Autocomplete (if possible)

- [ ] **Vim/Neovim Plugin** (2-3 days)
  - Syntax highlighting
  - Integration with ALE or Syntastic
  - Commands for validation

- [ ] **Emacs Mode** (2-3 days)
  - Syntax highlighting
  - Flycheck integration
  - Commands for validation

## Long Term (Months 4-6+)

### Advanced Features

- [ ] **Plugin System** (1-2 weeks)
  - Custom functions in Nickel
  - Rust-based plugins
  - Security sandbox for plugins
  - Plugin registry

- [ ] **LSP (Language Server Protocol)** (2-3 weeks)
  - IDE integration
  - Jump to definition
  - Find references
  - Rename refactoring
  - Diagnostics

- [ ] **Configuration Diffing** (1 week)
  ```bash
  bunsenite diff config1.ncl config2.ncl
  ```
  - Show differences between configs
  - Semantic diff (not just text)
  - Colorized output

- [ ] **Migration Tools** (1 week)
  ```bash
  bunsenite migrate from-json config.json > config.ncl
  bunsenite migrate from-yaml config.yaml > config.ncl
  bunsenite migrate from-toml config.toml > config.ncl
  ```
  - Convert JSON/YAML/TOML to Nickel
  - Preserve comments (best effort)
  - Type inference

### Ecosystem

- [ ] **Bunsenite Server** (2-3 weeks)
  - HTTP API for config parsing
  - GraphQL endpoint
  - Authentication/authorization
  - Caching
  - Metrics

- [ ] **Bunsenite Cloud** (Months, optional)
  - Managed config service
  - Version control for configs
  - Collaboration features
  - Audit logs
  - SLA guarantees

### Community

- [ ] **Contributor Onboarding** (Ongoing)
  - Mentor new contributors
  - "Good first issue" labels
  - Pair programming sessions
  - Recognition program

- [ ] **Community Events** (Quarterly)
  - Virtual meetups
  - Coding challenges
  - Hackathons
  - Conference presentations

## Stretch Goals

### Research & Innovation

- [ ] **Formal Verification** (Research project)
  - Prove correctness properties
  - Integration with SPARK or Coq
  - Verify no panics, no unsafe

- [ ] **Advanced Optimization** (Research project)
  - JIT compilation for Nickel
  - LLVM backend
  - Approach 100% native speed

- [ ] **Distributed Configuration** (Major feature)
  - CRDT-based config merging
  - Offline-first sync
  - Conflict resolution
  - Integration with SaltRover

## Metrics & Success Criteria

### Adoption Metrics (6 months)

- [ ] 1,000+ crates.io downloads
- [ ] 100+ GitHub/GitLab stars
- [ ] 50+ production users
- [ ] 10+ external contributors
- [ ] 5+ language bindings

### Quality Metrics

- [ ] 90%+ test coverage
- [ ] <10ms latency for typical configs
- [ ] Zero critical security vulnerabilities
- [ ] <5 open bugs at any time

### Community Health

- [ ] Average issue response time <48 hours
- [ ] Average PR review time <1 week
- [ ] 80%+ contributor satisfaction
- [ ] Active discussions/month >20

## Resources

### Time Estimates

- **Maintenance** (weekly): 2-5 hours
- **Feature development** (monthly): 20-40 hours
- **Community engagement** (monthly): 5-10 hours

### Funding

- [ ] Apply for grants (NLnet, SFC, etc.)
- [ ] Corporate sponsorships
- [ ] GitHub Sponsors
- [ ] Patreon/Open Collective

### Collaboration

- [ ] Partner with Nickel language team
- [ ] Collaborate with RSR framework
- [ ] Join TPCF community initiatives

## Timeline

```
Month 1: Publishing, announcements, initial improvements
Month 2: Features (validation, watch mode, REPL)
Month 3: Additional language bindings, tooling
Month 4-6: Advanced features, ecosystem growth
Month 6+: Research, innovation, sustainability
```

## Priorities

1. **User Value**: Features that help users solve real problems
2. **Quality**: Stability, performance, security
3. **Community**: Welcoming, inclusive, responsive
4. **Sustainability**: Long-term viability, funding, maintenance
5. **Innovation**: Push boundaries, explore new ideas

---

**Status**: Updated 2025-11-22

**Next Review**: Monthly or as needed

**Feedback**: [Open an issue](https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite/-/issues/new) or discussion
