use std::collections::HashMap;

use crate::api::windows::policy::{self, ApiRoute};
use crate::arch::Arch;
use crate::emu::Emu;

pub mod ntapi;
pub mod syscall32;
pub mod syscall64;

const X86_PROLOGUE: &[u8] = &[0xb8]; // mov eax, service number
const X64_PROLOGUE: &[u8] = &[0x4c, 0x8b, 0xd1, 0xb8]; // mov r10, rcx; mov eax, number

fn dispatch(emu: &mut Emu, number: u64, name: &str, handler: impl FnOnce(&mut Emu)) {
    if emu.api_policy.stopped() {
        return;
    }
    let previous = emu.api_policy.entry.take();
    policy::begin_call(emu, "ntdll", name, number, ApiRoute::Syscall);
    if emu.call_syscall_hook(number) && !emu.api_policy.stopped() {
        handler(emu);
    }
    policy::finish_call(emu);
    if !emu.api_policy.stopped() {
        emu.api_policy.entry = previous;
    }
}

/// Read service numbers from direct Nt exports of the loaded guest ntdll.
pub(crate) fn index_syscall_names(emu: &Emu) -> HashMap<u64, String> {
    let prologue = match emu.cfg.arch {
        Arch::X86 => X86_PROLOGUE,
        Arch::X86_64 => X64_PROLOGUE,
        Arch::Aarch64 => return HashMap::new(),
    };
    let Some(module) = emu.export_indexes.get_by_name("ntdll") else {
        return HashMap::new();
    };
    module
        .iter_for_dump()
        .into_iter()
        .filter_map(|(name, address)| {
            if address == 0 || !name.starts_with("Nt") {
                return None;
            }
            for (offset, byte) in prologue.iter().enumerate() {
                if emu.maps.read_byte(address.checked_add(offset as u64)?) != Some(*byte) {
                    return None;
                }
            }
            let number = emu
                .maps
                .read_dword(address.checked_add(prologue.len() as u64)?)?;
            Some((number as u64, name))
        })
        .collect()
}
