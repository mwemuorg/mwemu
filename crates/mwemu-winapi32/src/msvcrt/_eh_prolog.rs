use crate::emu;

pub fn _EH_prolog(emu: &mut emu::Emu) {
    let handler = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "msvcrt!_EH_prolog handler:0x{:x}", handler);
    emu.stack_push32(emu.regs().get_ebp() as u32);
}
