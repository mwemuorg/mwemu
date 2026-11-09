use crate::emu;
use crate::winapi::helper;

pub fn FindFirstVolumeA(emu: &mut emu::Emu) {
    let lpszVolumeName = emu.regs().rcx as u32;
    let _cchBufferLength = emu.regs().rdx as u32;

    let lpszVolumeName = emu.maps.read_string(lpszVolumeName as u64);

    log_red!(emu, "kernel32!FindFirstVolumeA {}", lpszVolumeName);

    emu.regs_mut().rax = helper::handler_create("FindFirstVolumeA");
}
