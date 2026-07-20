//! Sigma Racer frame map for the SocketCAN → VehicleState bridge.
//!
//! The live-capture path decodes raw CAN with a `FastDbc`, giving
//! `(message, signal, value)` tuples. Feeding those through this [`VssMap`]
//! yields VSS points, which [`VehicleState::apply_vss`] applies — so the
//! SocketCAN path builds the same `VehicleState` the WiFi/replay paths do, and
//! the shop-side anomaly detectors can run on it.

use std::sync::OnceLock;

use vss_map::VssMap;

/// Frame map bytes (keep in sync with wingman `schemas/can/sigma-racer.yaml`).
pub const SIGMA_RACER_FRAME_MAP: &str = include_str!("../data/sigma-racer.yaml");

/// The shared Sigma Racer VSS map, parsed once. Signals absent from the map
/// (e.g. when a non-Sigma DBC is loaded) translate to nothing.
pub fn sigma_vss_map() -> &'static VssMap {
    static MAP: OnceLock<VssMap> = OnceLock::new();
    MAP.get_or_init(|| {
        VssMap::from_frame_map_str(SIGMA_RACER_FRAME_MAP)
            .expect("embedded sigma-racer frame map must parse")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_map_parses_and_maps_known_signals() {
        let map = sigma_vss_map();
        let point = map
            .translate("ENGINE_STATUS", &("EngineRPM", 7450.0))
            .expect("EngineRPM must map");
        assert_eq!(
            point.path.as_str(),
            "Vehicle.Powertrain.CombustionEngine.Speed"
        );
    }
}
