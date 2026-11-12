use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn EnumSystemCodePagesA(emu: &mut emu::Emu) {
    let _lpCodePageEnumProc = WinApiAbi::arg(emu, 0) as u32;
    let _dwFlags = WinApiAbi::arg(emu, 1) as u32;

    log_red!(emu, "kernel32!EnumSystemCodePagesA");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn EnumSystemCodePagesW(emu: &mut emu::Emu) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(emu, "kernel32!EnumSystemCodePagesW '{}'  0x{:x}", s, arg2);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
