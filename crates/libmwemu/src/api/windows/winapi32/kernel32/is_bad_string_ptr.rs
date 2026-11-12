use crate::emu;
use crate::windows::constants::errors::*;

pub fn IsBadStringPtrA(emu: &mut emu::Emu) {
    is_bad_string_ptr(emu, "IsBadStringPtrA");
}

fn is_bad_string_ptr(emu: &mut emu::Emu, api: &str) {
    let _lpsz = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _ucchMax = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn IsBadStringPtrW(emu: &mut emu::Emu) {
    is_bad_string_ptr(emu, "IsBadStringPtrW");
}
