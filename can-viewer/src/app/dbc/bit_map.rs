//! Signal bit-occupancy grid for the DBC editor (Intel vs Motorola layout).

use crate::dto::{MessageInfo, SignalInfo};
use crate::{DbcBitBand as UiDbcBitBand, DbcBitCell as UiDbcBitCell, DbcBitRow as UiDbcBitRow};

/// Bits occupied by a signal (DBC Intel `@1` or Motorola `@0` layout).
pub(super) fn signal_occupied_bits(start_bit: u32, length: u32, byte_order: &str) -> Vec<u32> {
    if length == 0 {
        return Vec::new();
    }
    let big_endian = byte_order == "big_endian" || byte_order == "motorola";
    if big_endian {
        // Motorola: start_bit is MSB; walk toward LSB, wrapping to next byte's bit7.
        let mut bits = Vec::with_capacity(length as usize);
        let mut bit = start_bit;
        for _ in 0..length {
            bits.push(bit);
            if bit.is_multiple_of(8) {
                bit = bit.saturating_add(15);
            } else {
                bit = bit.saturating_sub(1);
            }
        }
        bits
    } else {
        // Intel: start_bit is LSB; consecutive ascending bit numbers.
        (start_bit..start_bit.saturating_add(length)).collect()
    }
}

/// Occupancy grid using each signal's byte order (Intel vs Motorola).
pub(super) fn build_bit_map(
    msg: &MessageInfo,
    dlc: u8,
    selected_signal: i32,
    live_selected: Option<&(u32, u32, String)>,
) -> (Vec<(String, Vec<UiDbcBitCell>)>, String) {
    let capacity = (dlc as usize).saturating_mul(8);

    let signal_start_len = |sig_i: usize, sig: &SignalInfo| -> (u32, u32) {
        if selected_signal == sig_i as i32 {
            if let Some((s, l, _)) = live_selected {
                return (*s, *l);
            }
        }
        (sig.start_bit, sig.length)
    };

    let signal_bits = |sig_i: usize, sig: &SignalInfo| -> Vec<u32> {
        let (start, length) = signal_start_len(sig_i, sig);
        signal_occupied_bits(start, length, &sig.byte_order)
    };

    let mut max_bit = capacity;
    for (sig_i, sig) in msg.signals.iter().enumerate() {
        for bit in signal_bits(sig_i, sig) {
            max_bit = max_bit.max(bit as usize + 1);
        }
    }
    let display_bytes = max_bit.div_ceil(8).max(dlc as usize);
    let bit_count = display_bytes * 8;

    let mut occupancy: Vec<u32> = vec![0; bit_count];
    let mut owner: Vec<i32> = vec![-1; bit_count];
    let mut selected_bits: std::collections::HashSet<u32> = std::collections::HashSet::new();

    for (sig_i, sig) in msg.signals.iter().enumerate() {
        let bits = signal_bits(sig_i, sig);
        if sig_i as i32 == selected_signal {
            selected_bits.extend(bits.iter().copied());
        }
        for bit in bits {
            let idx = bit as usize;
            if idx >= bit_count {
                continue;
            }
            occupancy[idx] = occupancy[idx].saturating_add(1);
            if owner[idx] < 0 || sig_i as i32 == selected_signal {
                owner[idx] = sig_i as i32;
            }
        }
    }

    // state: 0 free, 1 used, 2 selected, 3 conflict (overlap or past DLC)
    let mut bits: Vec<(i32, i32)> = vec![(0, -1); bit_count];
    for bit in 0..bit_count {
        let past_dlc = bit >= capacity;
        let overlap = occupancy[bit] >= 2;
        let is_selected = selected_bits.contains(&(bit as u32));

        let state = if (past_dlc && occupancy[bit] > 0) || overlap {
            3
        } else if past_dlc {
            0
        } else if is_selected {
            2
        } else if occupancy[bit] > 0 {
            1
        } else {
            0
        };

        let sig_idx = if is_selected {
            selected_signal
        } else {
            owner[bit]
        };
        bits[bit] = (state, sig_idx);
    }

    let used = (0..capacity).filter(|&b| occupancy[b] > 0).count();
    let free = capacity.saturating_sub(used);
    let overlap_bits = (0..capacity).filter(|&b| occupancy[b] >= 2).count();
    let past_dlc_bits = (capacity..bit_count).filter(|&b| occupancy[b] > 0).count();

    let (sel_start, sel_len, sel_name, sel_endian) = if selected_signal >= 0 {
        if let Some(sig) = msg.signals.get(selected_signal as usize) {
            let (start, length) = signal_start_len(selected_signal as usize, sig);
            let name = if let Some((_, _, n)) = live_selected {
                n.clone()
            } else {
                sig.name.clone()
            };
            let endian = if sig.byte_order == "big_endian" || sig.byte_order == "motorola" {
                "Motorola"
            } else {
                "Intel"
            };
            (start, length, name, endian)
        } else {
            (0, 0, String::new(), "")
        }
    } else {
        (0, 0, String::new(), "")
    };

    let mut summary = format!("DLC {dlc} · {capacity} bits · used {used} · free {free}");
    if overlap_bits > 0 {
        summary.push_str(&format!(" · overlap {overlap_bits}"));
    }
    if past_dlc_bits > 0 {
        summary.push_str(&format!(" · past DLC {past_dlc_bits}"));
    }
    if selected_signal >= 0 && !sel_name.is_empty() {
        let range = if sel_endian == "Motorola" {
            format!("start {sel_start} len {sel_len} {sel_endian}")
        } else {
            let end = sel_start.saturating_add(sel_len.saturating_sub(1));
            format!("{sel_start}–{end} {sel_endian}")
        };
        summary.push_str(&format!(" · {sel_name} [{range}]"));
    }

    let mut rows = Vec::with_capacity(display_bytes);
    for byte in 0..display_bytes {
        let mut cells = Vec::with_capacity(8);
        for display_col in 0..8 {
            let linear = byte * 8 + (7 - display_col);
            let (state, sig_idx) = bits.get(linear).copied().unwrap_or((0, -1));
            cells.push(UiDbcBitCell {
                state,
                signal_index: sig_idx,
            });
        }
        let label = if byte < dlc as usize {
            format!("Byte {byte}")
        } else {
            format!("+{byte}")
        };
        rows.push((label, cells));
    }

    (rows, summary)
}

/// Chunk byte strips into bands of up to 4 (four columns, wrap to next row).
pub(super) fn bit_bands_model(
    rows: &[(String, Vec<UiDbcBitCell>)],
) -> slint::ModelRc<UiDbcBitBand> {
    const COLS: usize = 4;
    let bands_model = slint::VecModel::default();
    for chunk in rows.chunks(COLS) {
        let bytes = slint::VecModel::default();
        for (label, cells) in chunk {
            let cell_model = slint::VecModel::default();
            for c in cells {
                cell_model.push(c.clone());
            }
            bytes.push(UiDbcBitRow {
                byte_label: label.clone().into(),
                cells: slint::ModelRc::new(cell_model),
            });
        }
        bands_model.push(UiDbcBitBand {
            bytes: slint::ModelRc::new(bytes),
        });
    }
    slint::ModelRc::new(bands_model)
}

#[cfg(test)]
mod bit_map_tests {
    use super::signal_occupied_bits;

    #[test]
    fn motorola_rpm_style_16_from_bit_7() {
        let bits = signal_occupied_bits(7, 16, "big_endian");
        assert_eq!(
            bits,
            vec![7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8]
        );
    }

    #[test]
    fn intel_oil_pressure_32_len_16() {
        let bits = signal_occupied_bits(32, 16, "little_endian");
        assert_eq!(bits, (32..48).collect::<Vec<_>>());
    }

    #[test]
    fn engine_data_signals_do_not_overlap() {
        // Matches sample.dbc EngineData packing
        let rpm = signal_occupied_bits(7, 16, "big_endian");
        let temp = signal_occupied_bits(23, 8, "big_endian");
        let throttle = signal_occupied_bits(31, 8, "big_endian");
        let oil = signal_occupied_bits(32, 16, "little_endian");
        let mut all = std::collections::HashSet::new();
        for set in [&rpm, &temp, &throttle, &oil] {
            for b in set {
                assert!(all.insert(*b), "overlap on bit {b}");
            }
        }
        assert_eq!(all.len(), 48);
    }
}
