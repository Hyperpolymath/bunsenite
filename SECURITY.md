# Security Policy

## Supported Versions

We take security seriously and provide security updates for the following versions:

| Version | Supported          | Notes                    |
| ------- | ------------------ | ------------------------ |
| 0.1.x   | :white_check_mark: | Current stable release   |
| < 0.1.0 | :x:                | Pre-release, not supported |

## Security Guarantees

Bunsenite provides the following security guarantees:

### Memory Safety

- **Zero `unsafe` blocks**: Enforced by `#![deny(unsafe_code)]` compiler directive
- **Rust ownership model**: Prevents use-after-free, double-free, and memory leaks
- **No null pointer dereferences**: Rust's type system eliminates this class of bugs
- **Bounds checking**: All array/vector accesses are bounds-checked

### Type Safety

- **Compile-time guarantees**: Type errors are caught before runtime
- **No implicit conversions**: Explicit type conversions required
- **Strong typing**: Prevents type confusion vulnerabilities

### Dependency Security

- **Minimal dependencies**: Only essential, well-audited crates
- **No network dependencies**: Offline-first design eliminates network attack surface
- **Pinned versions**: Dependencies locked to specific versions for reproducibility
- **Regular audits**: Dependencies audited using `cargo audit`

### Supply Chain Security

- **Reproducible builds**: Nix flake provides bit-for-bit reproducibility
- **Signed releases**: All releases are cryptographically signed (planned)
- **Transparent development**: All changes tracked in public Git repository
- **SBOM generation**: Software Bill of Materials available (planned)

## Reporting a Vulnerability

**Please do NOT report security vulnerabilities through public GitHub/GitLab issues.**

### Preferred Method

Report security vulnerabilities via:

1. **Email**: security@example.com <!-- TODO: Update with real email -->
2. **Encrypted email**: Use our PGP key (see `.well-known/security.txt`)
3. **GitLab Security**: Use GitLab's confidential issue feature

### What to Include

Please include:

- **Description**: Clear description of the vulnerability
- **Impact**: What an attacker could achieve
- **Reproduction**: Step-by-step instructions to reproduce
- **Affected versions**: Which versions are affected
- **Proposed fix**: If you have one (optional)
- **Disclosure timeline**: Your preferred disclosure timeline

### Response Timeline

- **Initial response**: Within 48 hours
- **Triage**: Within 1 week
- **Fix development**: Depends on severity (critical: days, low: weeks)
- **Public disclosure**: Coordinated with reporter, typically 90 days after fix

### Severity Levels

We use the following severity classifications:

#### Critical (CVSS 9.0-10.0)

- Remote code execution
- Privilege escalation to admin/root
- Authentication bypass

**Response**: Patch within 48 hours, immediate release

#### High (CVSS 7.0-8.9)

- SQL injection (not applicable to Bunsenite)
- Information disclosure of sensitive data
- Denial of service affecting availability

**Response**: Patch within 1 week, expedited release

#### Medium (CVSS 4.0-6.9)

- Cross-site scripting (XSS) (browser/WASM context)
- Information disclosure of non-sensitive data
- Low-impact denial of service

**Response**: Patch within 2 weeks, next regular release

#### Low (CVSS 0.1-3.9)

- Minor information leaks
- Best practice violations
- Theoretical attacks with no known exploit

**Response**: Patch within 30 days, next regular release

## Security Best Practices

### For Users

1. **Keep updated**: Always use the latest stable version
2. **Verify signatures**: Check release signatures (when available)
3. **Audit dependencies**: Run `cargo audit` regularly
4. **Minimal permissions**: Run with least privilege necessary
5. **Air-gapped environments**: Bunsenite works offline by design

### For Developers

1. **No `unsafe` code**: Never use `unsafe` blocks (enforced by compiler)
2. **Input validation**: Validate all external input
3. **Error handling**: Use `Result` types, avoid `unwrap()` in library code
4. **Dependency review**: Review new dependencies carefully
5. **Security testing**: Include security tests in test suite

## Known Limitations

### By Design

1. **Nickel evaluation**: Bunsenite evaluates Nickel code, which could contain:
   - Infinite loops (resource exhaustion)
   - Large memory allocations
   - Consider: Run evaluation in sandboxed environment for untrusted input

2. **File I/O**: File reading follows OS permissions
   - Does NOT escalate privileges
   - Respects filesystem boundaries

3. **WASM sandbox**: Browser WASM runs in sandbox, but:
   - Subject to browser security model
   - Can consume memory/CPU (denial of service)

### Mitigations

We provide:

- **Timeouts**: (Planned) Configurable evaluation timeouts
- **Memory limits**: (Planned) Configurable memory limits for evaluation
- **Resource monitoring**: (Planned) Track resource usage

## Security Audits

| Date       | Auditor | Scope         | Findings | Status     |
| ---------- | ------- | ------------- | -------- | ---------- |
| 2025-Q2    | Planned | Full codebase | N/A      | Scheduled  |

## Cryptography

Bunsenite does NOT implement cryptography. For cryptographic needs:

- Use established libraries (e.g., `ring`, `sodiumoxide`)
- Never roll your own crypto
- Follow NIST/IETF recommendations

## Compliance

- **OWASP Top 10**: N/A (not a web application)
- **CWE Top 25**: Memory safety issues prevented by Rust
- **GDPR**: No personal data collection
- **CCPA**: No personal data collection

## Security Tooling

We use:

- **`cargo audit`**: Check for known vulnerabilities in dependencies
- **`cargo clippy`**: Lint for security anti-patterns
- **`cargo deny`**: Check licenses and security advisories
- **GitLab Security Scanner**: Automated SAST in CI/CD
- **Dependabot**: (Planned) Automated dependency updates

## Contact

- **Security email**: security@example.com <!-- TODO: Update -->
- **PGP key**: See `.well-known/security.txt`
- **Security.txt**: See `.well-known/security.txt` (RFC 9116 compliant)

## Attribution

We believe in responsible disclosure and will credit security researchers who:

- Report vulnerabilities responsibly
- Allow coordinated disclosure
- Follow our security policy

Credits will be listed in:
- CHANGELOG.md
- Release notes
- SECURITY.md (this file)

## Legal

Security research conducted in good faith will not result in legal action, provided:

- You respect our disclosure timeline
- You do not exploit vulnerabilities beyond proof-of-concept
- You do not access user data or disrupt service
- You comply with applicable laws

We support security researchers and the white-hat community.

---

**Last updated**: 2025-11-22
**Version**: 0.1.0
