use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn SetInformationJobObject(emu: &mut emu::Emu) {
    let _hJob = WinApiAbi::arg(emu, 0) as u32;
    let _JobObjectInformationClass = WinApiAbi::arg(emu, 1) as u32;
    let _lpJobObjectInformation = WinApiAbi::arg(emu, 2) as u32;
    let _cbJobObjectInformationLength = WinApiAbi::arg(emu, 3) as u32;

    log_red!(emu, "kernel32!SetInformationJobObject");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
