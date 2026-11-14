use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GetBinaryType(emu: &mut emu::Emu) {
    let handle = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    log_red!(
        emu,
        "kernel32!GetBinaryType h:0x{:x} arg2:0x{:x}",
        handle,
        arg2
    );
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn GetBinaryTypeA(emu: &mut emu::Emu) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let s = emu.maps.read_string(str_ptr);
    log_red!(emu, "kernel32!GetBinaryTypeA '{}'  0x{:x}", s, arg2);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn GetBinaryTypeW(emu: &mut emu::Emu) {
    let lpApplicationName = WinApiAbi::arg(emu, 0) as u32;
    let _lpBinaryType = WinApiAbi::arg(emu, 1) as u32;

    let lpApplicationName = emu.maps.read_wide_string(lpApplicationName as u64);

    log_red!(emu, "kernel32!GetBinaryTypeW {}", lpApplicationName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
