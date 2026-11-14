use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn CreateFileMappingNumaA(emu: &mut emu::Emu) {
    let _hFile = WinApiAbi::arg(emu, 0) as u32;
    let _lpFileMappingAttributes = WinApiAbi::arg(emu, 1) as u32;
    let _flProtect = WinApiAbi::arg(emu, 2) as u32;
    let _dwMaximumSizeHigh = WinApiAbi::arg(emu, 3) as u32;
    let _dwMaximumSizeLow = WinApiAbi::arg(emu, 4) as u32;
    let lpName = WinApiAbi::arg(emu, 5) as u32;
    let _nndPreferred = WinApiAbi::arg(emu, 6) as u32;

    let lpName = emu.maps.read_string(lpName as u64);

    log_red!(emu, "kernel32!CreateFileMappingNumaA {}", lpName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}

pub fn CreateFileMappingNumaW(emu: &mut emu::Emu) {
    for _ in 0..7u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreateFileMappingNumaW");
    emu.regs_mut().rax = helper::handler_create("CreateFileMappingNumaW");
}
