# ADR 006: Apache License 2.0

## Status: Accepted

## Date: 2026-03-01

## Context

900Invoice is open-source software. The choice of license determines how individuals, businesses, and organizations can use, modify, and distribute the software. This decision affects:

1. The ability of businesses to adopt and integrate 900Invoice
2. The ability of developers to build commercial products on top of 900Invoice
3. Ecosystem consistency with 900PDF and 900CRM
4. Compatibility with potential future enterprise features or dual-licensing

### Licenses Considered

**GPL v3 (GNU General Public License)**

The GPL requires that any derivative work or software that incorporates GPL-licensed code must also be distributed under the GPL. This is a "copyleft" license.

*Why we rejected it:* Our goal is maximum adoption and utility, particularly in developing economies. A freelancer in Lagos who wants to build a custom billing system on top of 900Invoice should not be legally required to open-source their entire business application. The GPL would create a barrier to commercial adoption that works against our mission.

**MIT License**

A permissive license that allows nearly unrestricted use, modification, and distribution. Very widely used.

*Why we considered it:* Simple, well-understood, maximum permissiveness.

*Why we chose Apache over MIT:* Apache 2.0 includes an explicit patent grant that MIT does not. The patent grant provides users with protection against patent claims from contributors — an important protection for business users.

**Apache License 2.0**

A permissive open-source license similar to MIT but with:
1. An explicit patent license grant from all contributors to all users
2. A patent retaliation clause (if you sue a contributor for patent infringement related to the project, your patent license from that contributor terminates)
3. Requirement to include the NOTICE file when redistributing (minimal, non-burdensome)

*Why we chose it:*
- **Patent protection**: Explicit patent grant protects business users
- **Permissiveness**: No copyleft requirements — businesses can build proprietary products on top of 900Invoice
- **Ecosystem consistency**: All 900 Labs open-source projects use Apache 2.0
- **Enterprise familiarity**: Enterprise legal teams are familiar with and comfortable with Apache 2.0
- **CNCF/ASF standard**: Apache 2.0 is the de facto standard for enterprise open-source (Kubernetes, TensorFlow, TypeScript, etc.)

**AGPL (Affero GPL)**

A variant of GPL that additionally requires the source to be made available when software is offered as a service over a network.

*Why we rejected it:* 900Invoice is a desktop application with no server component, making the AGPL's additional restrictions moot in our primary use case. The AGPL is sometimes used to prevent SaaS companies from building closed-source cloud wrappers around open-source tools — not a concern for a desktop-native application.

## Decision

**Apache License 2.0**, Copyright 2026 900 Labs.

The full license text is in the `LICENSE` file at the project root. Every source file includes the standard Apache 2.0 SPDX identifier:

```rust
// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 900 Labs
```

## Consequences

### Positive
- Maximum freedom for users, businesses, and developers to adopt and build on 900Invoice
- Explicit patent protection for all users — important for business adoption
- Consistent with the broader 900 Labs ecosystem
- Compatible with other Apache 2.0 and MIT dependencies (no license conflicts)
- No restrictions on commercial use, modification, or redistribution

### Obligations for Users and Distributors

Users and distributors must:
1. Include a copy of the Apache License 2.0 when distributing
2. Include attribution notices from the `NOTICE` file (if one exists)
3. State significant changes made to the original files

Users and distributors are **not** required to:
1. Open-source derivative works or integrations
2. Pay any fees or royalties
3. Seek permission from 900 Labs for commercial use

### For Contributors

By submitting a contribution to this project, you agree to license your contribution under the Apache License 2.0. This is the standard Contributor License Agreement (CLA) implied by the project's license. We do not require a separate CLA document.

### Compatibility

Apache 2.0 is compatible with:
- MIT, BSD 2-Clause, BSD 3-Clause
- GPL v3 (when incorporating Apache 2.0 code into a GPL v3 project)
- MPL 2.0

Apache 2.0 is **not** directly compatible with:
- GPL v2 (the patent termination clause creates a conflict)

All current dependencies of 900Invoice are licensed under Apache 2.0, MIT, or BSD licenses, which are all compatible.
