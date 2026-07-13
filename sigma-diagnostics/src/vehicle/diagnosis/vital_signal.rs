/// One vital for the Mechanic diagnosis panel (VSS-aligned labels).
#[derive(Debug, Clone, Default)]
pub struct VitalSignal {
    pub name: String,
    pub value: String,
    pub unit: String,
}
