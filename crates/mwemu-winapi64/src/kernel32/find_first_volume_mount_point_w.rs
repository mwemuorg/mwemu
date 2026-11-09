use crate::emu;
use crate::winapi::helper;

pub fn FindFirstVolumeMountPointW(emu: &mut emu::Emu) {
    let lpszRootPathName = emu.regs().rcx as u32;
    let _lpszVolumeMountPoint = emu.regs().rdx as u32;
    let _cchBufferLength = emu.regs().r8 as u32;

    let lpszRootPathName = emu.maps.read_wide_string(lpszRootPathName as u64);

    log_red!(
        emu,
        "kernel32!FindFirstVolumeMountPointW {}",
        lpszRootPathName
    );

    emu.regs_mut().rax = helper::handler_create("FindFirstVolumeMountPointW");
}
