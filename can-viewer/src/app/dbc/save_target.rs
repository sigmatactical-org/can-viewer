/// Where [`super::DbcController::run_save`] should write the DBC.
pub(super) enum SaveTarget {
    ExistingPathOrPrompt,
    PickPath,
}
