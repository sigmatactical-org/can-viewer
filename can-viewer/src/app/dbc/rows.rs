//! Build Slint table rows from DBC domain types.

use crate::dto::{DbcInfo, MessageInfo, NodeInfo};
use crate::{
    DbcMessageRow as UiDbcMessageRow, DbcNodeRow as UiDbcNodeRow, DbcSignalRow as UiDbcSignalRow,
};

use super::endian::endian_label;

pub(super) fn message_row(m: &MessageInfo) -> UiDbcMessageRow {
    UiDbcMessageRow {
        id: format!("0x{:X}", m.id).into(),
        name: m.name.clone().into(),
        dlc: m.dlc.to_string().into(),
        sender: m.sender.clone().into(),
        signal_count: m.signals.len().to_string().into(),
    }
}

pub(super) fn node_row(n: &NodeInfo) -> UiDbcNodeRow {
    UiDbcNodeRow {
        name: n.name.clone().into(),
        comment: n.comment.clone().unwrap_or_default().into(),
    }
}

pub(super) fn selected_message_signals(info: &DbcInfo, msg_idx: i32) -> Vec<UiDbcSignalRow> {
    info.messages
        .get(msg_idx as usize)
        .map(|m| {
            m.signals
                .iter()
                .map(|s| UiDbcSignalRow {
                    name: s.name.clone().into(),
                    start_bit: s.start_bit.to_string().into(),
                    length: s.length.to_string().into(),
                    byte_order: endian_label(&s.byte_order).into(),
                    factor: s.factor.to_string().into(),
                    unit: s.unit.clone().into(),
                })
                .collect()
        })
        .unwrap_or_default()
}
