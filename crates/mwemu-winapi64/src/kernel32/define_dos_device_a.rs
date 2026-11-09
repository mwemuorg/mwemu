
use crate::windows::constants::errors::*;
use crate::emu;

pub fn DefineDosDeviceA(emu: &mut emu::Emu) {
    let _dwFlags = emu.regs().rcx as u32;
    let lpDeviceName = emu.regs().rdx as u32;
    let lpTargetPath = emu.regs().r8 as u32;

    let lpDeviceName = emu.maps.read_string(lpDeviceName as u64);
    let lpTargetPath = emu.maps.read_string(lpTargetPath as u64);

    log_red!(
        emu,
        "kernel32!DefineDosDeviceA {} {}",
        lpDeviceName,
        lpTargetPath
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
