use crate::emu;
use crate::winapi::helper;

pub fn FindNextVolumeMountPointW(emu: &mut emu::Emu) {
    let _hFindVolumeMountPoint = emu.regs().rcx as u32;
    let _lpszVolumeMountPoint = emu.regs().rdx as u32;
    let _cchBufferLength = emu.regs().r8 as u32;

    log_red!(emu, "kernel32!FindNextVolumeMountPointW");

    emu.regs_mut().rax = helper::handler_create("FindNextVolumeMountPointW");
}
