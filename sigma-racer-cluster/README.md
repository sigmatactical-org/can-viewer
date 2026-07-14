# sigma-racer-cluster

On-vehicle instrument cluster for the Sigma Racer — a thin
[Slint](https://slint.dev) UI shell over the `sigma-diagnostics` domain
crate.

## License: GPL-3.0-only (and why)

This crate is the **one exception** to the repository's MIT OR
Apache-2.0 licensing. It is licensed **GPL-3.0-only** (see
[LICENSE](LICENSE)) because it links Slint under Slint's GPL-3.0-only
option, which permits deployment on embedded hardware without a
commercial Slint license.

The boundary is deliberate and must stay thin:

- **Logic lives downstream.** CAN decode, protocols, vehicle state, and
  anything reusable belongs in `sigma-diagnostics` (MIT OR Apache-2.0).
  This crate only maps domain types onto Slint properties. If a change
  here starts growing logic, move it down first.
- **Contributions are dual-licensed.** By contributing to this crate you
  agree your contribution is licensed MIT OR Apache-2.0 (as with the
  rest of the repository), in addition to being distributed here under
  GPL-3.0-only. This keeps the maintainers free to move code into the
  permissive crates or relicense the UI layer later.
- **No proprietary artwork.** Sigma Tactical Group names, logos, and
  artwork are proprietary (see [BRANDING.md](../BRANDING.md)) and are
  not included in this crate. The open layout ships neutral styling;
  official builds inject brand assets at build time.

## Shipping obligations

Conveying a device with this software installed is distribution under
GPLv3. In particular:

- **Corresponding source** for this crate (and the complete combined
  work) must accompany the device or be available by written offer.
- **Installation Information (§6):** on a consumer product, owners must
  be able to install modified versions. Firmware signing/OTA design
  must include an owner-unlock path for the cluster image.

## Run (development)

```sh
cargo run -p sigma-racer-cluster
```

Renders the placeholder layout in the disconnected state. Vehicle-bus
wiring (SocketCAN → `DiagnosisSnapshot`) is not implemented yet.
