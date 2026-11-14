use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub(super) fn exit(emu: &mut emu::Emu) {
    let status = WinApiAbi::arg(emu, 0) as u32;
    log_red!(emu, "msvcrt!exit {}", status);
    emu.stop();
}
