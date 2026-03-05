# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 1.0.x   | Yes       |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through GitHub Issues.** Issues are public, and disclosing a vulnerability publicly before a fix is available puts all users at risk.

Instead, email us at **[security@900labs.com](mailto:security@900labs.com)**.

### What to Include in Your Report

Please include as much of the following as you can:

- **Description**: A clear description of the vulnerability and its potential impact
- **Steps to reproduce**: Step-by-step instructions to reproduce the issue
- **Proof of concept**: Code, a script, or a screenshot demonstrating the vulnerability (if applicable)
- **Affected version**: The 900Invoice version you tested against
- **Your environment**: Operating system, OS version
- **Suggested fix**: If you have ideas about how to fix it (optional but appreciated)

The more detail you provide, the faster we can assess and fix the issue.

### Response Timeline

| Stage | Timeline |
|-------|----------|
| Acknowledgment | Within 48 hours of receiving your report |
| Initial assessment | Within 7 days |
| Fix development | Depends on severity (Critical: 14 days, High: 30 days, Medium: 60 days) |
| Release and disclosure | After fix is released and users have had time to update |

We will keep you informed throughout the process. If you do not receive an acknowledgment within 48 hours, please follow up via email.

### Coordinated Disclosure

We follow responsible disclosure. Please:

1. Give us a reasonable time to fix the vulnerability before any public disclosure
2. Make a good-faith effort not to access or modify other users' data
3. Do not perform actions that could harm users or the project (e.g., destroying data, distributing exploits)

In return, we will:

1. Acknowledge your contribution (with your permission) in the security advisory and release notes
2. Work with you to understand and fix the issue as quickly as possible
3. Notify you when the fix is released

### Scope

The following are **in scope** for security reports:

- **SQLite injection**: Any input that can result in unintended SQL execution
- **Path traversal**: Any input that can cause file reads or writes outside expected directories (e.g., backup/restore, CSV import)
- **Data integrity**: Bugs that silently corrupt financial data (invoice amounts, tax calculations)
- **Arbitrary code execution**: Any vector that allows executing code beyond the expected application behavior
- **File system access**: Unexpected reads or writes to files outside the application's data directory
- **IPC boundary issues**: Tauri commands that fail to validate inputs and can be abused by malicious frontend code

The following are **out of scope**:

- Social engineering attacks
- Physical access attacks
- Denial of service (DoS) — 900Invoice is a single-user desktop application
- Vulnerabilities in dependencies that have already been publicly disclosed and for which we have not yet issued a fix (though we appreciate reports that help us prioritize)
- Theoretical vulnerabilities without a proof of concept

### Note on Offline Architecture

900Invoice is an offline-first desktop application. It does not have a network-accessible server, API, or web interface. The attack surface is primarily:

1. **The SQLite database file** — can be read by any process with file system access under the same user account
2. **File operations** — backup, restore, CSV import/export
3. **The Tauri IPC bridge** — between the Svelte frontend and Rust backend

We do not consider it a vulnerability that the database file is readable by the local user, since it is their own data. We do consider it a vulnerability if another process on the same machine could inject malicious content that causes 900Invoice to act outside its intended scope.

## Security Acknowledgments

We gratefully acknowledge responsible security disclosures. Researchers who report valid vulnerabilities will be credited in the release notes for the patched version (with permission).

---

Contact: [security@900labs.com](mailto:security@900labs.com)
Website: [900labs.com](https://www.900labs.com)
