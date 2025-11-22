# Maintainers

This document lists the maintainers of the Bunsenite project and describes the maintenance structure.

## Current Maintainers

### Core Team (Perimeter 1)

These individuals have write access to the main repository and make final decisions on merges and releases.

- **Campaign for Cooler Coding and Programming** (@cccp)
  - Role: Lead Maintainer, Project Founder
  - Email: maintainers@example.com <!-- TODO: Update -->
  - PGP: See `.well-known/security.txt`
  - Focus: Overall architecture, releases, community

### Trusted Contributors (Perimeter 2)

These individuals have demonstrated consistent quality contributions and may have specialized access or responsibilities.

*(Currently none - invitations extended based on sustained contributions)*

## Contribution Perimeters (TPCF)

This project uses the **Tri-Perimeter Contribution Framework**:

### Perimeter 1: Core Maintainers
- **Access**: Full write access
- **Responsibilities**:
  - Review and merge PRs
  - Release management
  - Security response
  - Community moderation
  - Strategic direction
- **Membership**: By invitation, based on sustained commitment and expertise

### Perimeter 2: Trusted Contributors
- **Access**: Some specialized permissions (e.g., CI configuration, docs)
- **Responsibilities**:
  - Detailed code review
  - Mentoring new contributors
  - Area-specific expertise
  - Triage issues
- **Membership**: By invitation from Perimeter 1, based on consistent quality contributions

### Perimeter 3: Community Sandbox
- **Access**: Open to all
- **Responsibilities**:
  - Submit issues and PRs
  - Participate in discussions
  - Help other users
- **Membership**: Automatic for all contributors

## Areas of Responsibility

### Rust Core
- **Lead**: Core Team
- **Focus**: `src/lib.rs`, `src/loader.rs`, `src/error.rs`
- **Reviewers**: Core Team

### WASM Bindings
- **Lead**: Core Team
- **Focus**: `src/wasm.rs`, WASM build system
- **Reviewers**: Core Team

### FFI Bindings
- **Lead**: Core Team (seeking volunteers)
- **Focus**: `bindings/deno/`, `bindings/rescript/`, Zig layer
- **Reviewers**: Core Team

### CLI
- **Lead**: Core Team
- **Focus**: `src/main.rs`, user experience
- **Reviewers**: Core Team

### Documentation
- **Lead**: Core Team (help wanted!)
- **Focus**: README, CLAUDE.md, API docs, examples
- **Reviewers**: Any maintainer

### Infrastructure
- **Lead**: Core Team
- **Focus**: CI/CD, Justfile, Nix flake, releases
- **Reviewers**: Core Team

### Security
- **Lead**: Core Team
- **Contact**: security@example.com <!-- TODO: Update -->
- **Focus**: Vulnerability response, security audits, dependency audits
- **Reviewers**: Core Team only

## Maintenance Policies

### Code Review

- **Required**: At least 1 maintainer approval for all PRs
- **Self-merge**: Core team may merge own PRs for minor changes (typos, formatting)
- **Security**: Security PRs require 2 approvals
- **Breaking changes**: Require discussion and 2 approvals

### Release Process

1. **Version bump**: Update `Cargo.toml`, `CHANGELOG.md`
2. **Testing**: All tests must pass
3. **Documentation**: Update docs as needed
4. **Tag**: Create git tag `vX.Y.Z`
5. **Release**: Create GitLab release with notes
6. **Publish**: Publish to crates.io
7. **Announce**: Announce in discussions/issues

### Issue Triage

- **Labeling**: Apply appropriate labels (`bug`, `enhancement`, `documentation`, etc.)
- **Priority**: Assign priority (`P0`-`P3`)
- **Assignment**: Assign to maintainer or leave unassigned for community
- **Response time**: Aim for initial response within 1 week

### Security Response

- **Initial response**: Within 48 hours
- **Triage**: Within 1 week
- **Fix**: According to severity (see SECURITY.md)
- **Disclosure**: Coordinated, typically 90 days after fix

## Becoming a Maintainer

### Path to Perimeter 2 (Trusted Contributor)

We look for:

- **Consistent contributions**: Regular, quality contributions over 3+ months
- **Code quality**: Well-tested, documented, follows conventions
- **Community**: Helpful in discussions, reviews others' PRs
- **Alignment**: Understands and embodies project values (reversibility, emotional safety, political autonomy)

**Process**:
1. Core team discusses potential invitation
2. Invitation extended via private message
3. 1-month trial period
4. Full membership if successful

### Path to Perimeter 1 (Core Maintainer)

We look for:

- **Sustained commitment**: 6+ months of active, quality participation
- **Deep expertise**: Domain knowledge in core areas
- **Leadership**: Mentors others, drives initiatives
- **Trust**: Demonstrated judgment and alignment with project values

**Process**:
1. Nominated by existing core maintainer
2. Discussion among core team
3. Unanimous approval required
4. Onboarding period with gradual permission increase

## Stepping Down

Maintainers may step down at any time:

- **Voluntary**: No explanation needed, though appreciated
- **Inactive**: After 6 months of inactivity, we may reach out to confirm status
- **Emeritus**: Former maintainers are honored and may be consulted

**Process**:
1. Notify core team
2. Remove permissions
3. Update MAINTAINERS.md
4. Thank you! 🎉

## Conflict Resolution

### Technical Disagreements

1. **Discussion**: Discuss in issue/MR
2. **Evidence**: Present evidence and rationale
3. **Consensus**: Aim for consensus
4. **Vote**: If no consensus, core team votes (simple majority)
5. **Document**: Document decision and rationale

### Interpersonal Conflicts

1. **Direct**: Speak directly with the person (if safe)
2. **Mediation**: Request mediation from another maintainer
3. **Code of Conduct**: File CoC complaint if needed
4. **Resolution**: Follow CoC enforcement guidelines

## Contact

- **General**: maintainers@example.com <!-- TODO: Update -->
- **Security**: security@example.com <!-- TODO: Update -->
- **Code of Conduct**: conduct@example.com <!-- TODO: Update -->
- **GitLab**: [@cccp](https://gitlab.com/cccp)

## Acknowledgments

Thank you to all contributors, whether Perimeter 1, 2, or 3. Every contribution matters!

Special thanks to:
- Nickel language team (nickel-lang-core)
- RSR Framework contributors
- TPCF community
- All early adopters and testers

---

**Last updated**: 2025-11-22
**Version**: 1.0.0
