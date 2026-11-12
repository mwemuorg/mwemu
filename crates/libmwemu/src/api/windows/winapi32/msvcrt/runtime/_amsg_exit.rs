use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn _amsg_exit(emu: &mut emu::Emu) {
    let code = WinApiAbi::arg(emu, 0) as u32;
    log_red!(emu, "msvcrt!_amsg_exit code:{}", code);
    emu.stop();
}
