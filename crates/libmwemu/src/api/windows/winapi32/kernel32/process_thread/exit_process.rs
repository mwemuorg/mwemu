use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn ExitProcess(emu: &mut emu::Emu) {
    let code = WinApiAbi::arg(emu, 0) as u32;

    log_red!(emu, "kernel32!ExitProcess code: {}", code);
    emu.stack_pop32(false);

    emu.stop();
}
