use crate::emu;

pub fn _EH_prolog3(emu: &mut emu::Emu, api: &str) {
    let cookie = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "msvcrt!{} cookie:0x{:x}", api, cookie);
    emu.stack_push32(emu.regs().get_ebp() as u32);
}
