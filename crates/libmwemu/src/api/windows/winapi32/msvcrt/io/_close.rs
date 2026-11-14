use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn _close(emu: &mut emu::Emu) {
    let fd = WinApiAbi::arg(emu, 0);
    let name = helper::handler_get_uri(fd);
    let ok = helper::handler_close(fd);
    log_red!(emu, "msvcrt!_close fd:{} '{}' ok:{}", fd, name, ok);
    emu.regs_mut().rax = if ok { 0 } else { u64::MAX };
}
