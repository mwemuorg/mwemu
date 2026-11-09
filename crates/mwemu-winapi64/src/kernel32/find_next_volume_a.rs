use crate::emu;
use crate::winapi::helper;

pub fn FindNextVolumeA(emu: &mut emu::Emu) {
    let _hFindVolume = emu.regs().rcx as u32;
    let lpszVolumeName = emu.regs().rdx as u32;
    let _cchBufferLength = emu.regs().r8 as u32;

    let lpszVolumeName = emu.maps.read_string(lpszVolumeName as u64);

    log_red!(emu, "kernel32!FindNextVolumeA {}", lpszVolumeName);

    emu.regs_mut().rax = helper::handler_create("FindNextVolumeA");
}
