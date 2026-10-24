use crate::emu;
use crate::windows::constants;

pub fn SetCalendarInfoA(emu: &mut emu::Emu) {
    let _Locale = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _Calendar = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _CalType = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _lpCalData = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    log_red!(emu, "kernel32!SetCalendarInfoA");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
