use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn IsBadStringPtrA(emu: &mut emu::Emu) {
    is_bad_string_ptr(emu, "IsBadStringPtrA");
}

fn is_bad_string_ptr(emu: &mut emu::Emu, api: &str) {
    let _lpsz = WinApiAbi::arg(emu, 0) as u32;
    let _ucchMax = WinApiAbi::arg(emu, 1) as u32;

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn IsBadStringPtrW(emu: &mut emu::Emu) {
    is_bad_string_ptr(emu, "IsBadStringPtrW");
}
