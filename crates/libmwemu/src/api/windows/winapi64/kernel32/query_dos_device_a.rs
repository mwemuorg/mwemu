
use crate::windows::constants::errors::*;
use crate::emu;

pub fn QueryDosDeviceA(emu: &mut emu::Emu) {
    let lpDeviceName = emu.regs().rcx as u32;
    let lpTargetPath = emu.regs().rdx as u32;
    let _ucchMax = emu.regs().r8 as u32;

    let lpDeviceName = emu.maps.read_string(lpDeviceName as u64);
    let lpTargetPath = emu.maps.read_string(lpTargetPath as u64);

    log_red!(
        emu,
        "kernel32!QueryDosDeviceA {} {}",
        lpDeviceName,
        lpTargetPath
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
