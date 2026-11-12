use crate::emu;
use crate::windows::constants::errors::*;

pub fn WriteConsoleInputVDMA(emu: &mut emu::Emu) {
    write_console_input_vdm(emu, "WriteConsoleInputVDMA");
}

fn write_console_input_vdm(emu: &mut emu::Emu, api: &str) {
    let _a1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _a2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _a3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _a4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

pub fn WriteConsoleInputVDMW(emu: &mut emu::Emu) {
    write_console_input_vdm(emu, "WriteConsoleInputVDMW");
}
