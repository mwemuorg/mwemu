use crate::emu;
use crate::winapi::helper;

pub(super) fn fclose(emu: &mut emu::Emu) {
    let file = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let filename = helper::handler_get_uri(file);
    helper::handler_close(file);
    log_red!(emu, "msvcrt!fclose '{}' handle:0x{:x}", filename, file);
    emu.regs_mut().rax = 0;
}
