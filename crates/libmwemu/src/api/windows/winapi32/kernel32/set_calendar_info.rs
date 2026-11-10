use crate::emu;
use crate::windows::constants::errors::*;

pub fn SetCalendarInfoA(emu: &mut emu::Emu) {
    let _Locale = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _Calendar = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _CalType = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _lpCalData = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    log_red!(emu, "kernel32!SetCalendarInfoA");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

pub fn SetCalendarInfoW(emu: &mut emu::Emu) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(emu, "kernel32!SetCalendarInfoW '{}'  0x{:x}", s, arg2);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
