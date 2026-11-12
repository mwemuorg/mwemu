use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub(super) fn fflush(emu: &mut emu::Emu) {
    let file = WinApiAbi::arg(emu, 0) as u32;

    let filename = helper::handler_get_uri(file as u64);

    log_red!(emu, "msvcrt!fflush `{}`", filename);

    //emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}
