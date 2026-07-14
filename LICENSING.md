# Licensing

## Brand & artwork

The Sigma Tactical Group name, logos, marks, artwork, and visual identity
(including assets under `ui/assets/`) are **proprietary**. They are **not**
covered by the software licenses below. See [BRANDING.md](BRANDING.md).

## Software

The software is licensed under either of:

- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE))
- **MIT License** ([LICENSE-MIT](LICENSE-MIT))

at your option.

**Exception:** the `sigma-racer-cluster` crate is licensed
**GPL-3.0-only** ([sigma-racer-cluster/LICENSE](sigma-racer-cluster/LICENSE)).
It links Slint under Slint's GPL option so the instrument cluster can run
on embedded hardware without a commercial Slint license. Reusable logic
stays in the MIT/Apache crates; see
[sigma-racer-cluster/README.md](sigma-racer-cluster/README.md) for the
boundary rules and device-shipping obligations.

### Slint (UI toolkit)

The desktop apps (`can-viewer`, `sigma-racer-mechanic`) distribute under
the **Slint Royalty-Free License** (attribution in the in-app About tab).
The cluster uses Slint's **GPL-3.0-only** option instead.

This means you can use sigma-diagnostics in:
- Open-source projects (any license)
- Personal projects
- Educational projects
- Commercial projects (with attribution)

**Requirements:**
- Include copyright notices
- Include license files
- State any significant changes
- Third-party crate notices are listed in the in-app **About** tab and generated at build time in `target/*/build/can-viewer-*/out/THIRD_PARTY_NOTICES.txt`

## License Compatibility

### Compatible with:
- MIT
- Apache-2.0
- BSD
- ISC
- Most permissive licenses

### May have conflicts with:
- GPL (check compatibility carefully)
- AGPL (check compatibility carefully)
- LGPL (check compatibility)

## Contributions

By contributing to sigma-diagnostics, you agree that your contributions will be
licensed under MIT OR Apache-2.0 (at the recipient's option), the same as
the rest of the software.

## Questions?

If you're unsure which license applies to your use case, please contact
us for clarification.
