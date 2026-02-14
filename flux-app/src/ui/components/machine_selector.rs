use leptos::prelude::*;
use crate::shared::models::{MachineType, Pattern};

/// Convert MachineType to 2-3 letter abbreviation for compact display
fn machine_abbreviation(machine: MachineType) -> &'static str {
    match machine {
        MachineType::OneShot => "OS",
        MachineType::Werp => "WP",
        MachineType::Slice => "SL",
        MachineType::FmTone => "FM",
        MachineType::Subtractive => "SUB",
        MachineType::TonverkBus => "TNV",
        MachineType::MidiCC => "CC",
    }
}

/// Convert MachineType to full name for dropdown options
fn machine_full_name(machine: MachineType) -> &'static str {
    match machine {
        MachineType::OneShot => "OneShot",
        MachineType::Werp => "Werp",
        MachineType::Slice => "Slice",
        MachineType::FmTone => "FmTone",
        MachineType::Subtractive => "Subtractive",
        MachineType::TonverkBus => "TonverkBus",
        MachineType::MidiCC => "MidiCC",
    }
}

/// Get all machine types in order for dropdown
fn all_machine_types() -> [MachineType; 7] {
    [
        MachineType::OneShot,
        MachineType::Werp,
        MachineType::Slice,
        MachineType::FmTone,
        MachineType::Subtractive,
        MachineType::TonverkBus,
        MachineType::MidiCC,
    ]
}
