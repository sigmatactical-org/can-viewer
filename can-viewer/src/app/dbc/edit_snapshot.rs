use crate::dto::DbcInfo;

/// Snapshot taken when entering edit / before New — restored by Undo.
#[derive(Clone)]
pub(super) struct EditSnapshot {
    pub(super) info: DbcInfo,
    pub(super) path: Option<String>,
    pub(super) dirty: bool,
}
