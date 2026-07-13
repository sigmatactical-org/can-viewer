/// A single vehicle setting key/value, with a read-only marker.
#[derive(Debug, Clone)]
pub struct VehicleSetting {
    pub key: String,
    pub value: String,
    pub read_only: bool,
}
