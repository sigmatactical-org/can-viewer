//! DBC editor tab controller.
//!
//! [`DbcController`] owns the tab; its private snapshot/save-target types and
//! the row-building, bit-map, edit-apply, and endian helpers each live in
//! their own file.

mod bit_map;
mod dbc_controller;
mod edit_snapshot;
mod edits;
mod endian;
mod rows;
mod save_target;

pub use dbc_controller::DbcController;
