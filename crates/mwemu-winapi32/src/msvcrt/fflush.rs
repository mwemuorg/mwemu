use crate::emu;
use crate::winapi::helper;

pub(super) fn fflush(emu: &mut emu::Emu) {
    let file = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    let filename = helper::handler_get_uri(file as u64);

    log_red!(emu, "msvcrt!fflush `{}`", filename);

    //emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}
