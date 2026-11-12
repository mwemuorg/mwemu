use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn WSAGetQOSByName(emu: &mut emu::Emu) {
    let sock = WinApiAbi::arg(emu, 0);
    let name_ptr = WinApiAbi::arg(emu, 1);
    let qos_ptr = WinApiAbi::arg(emu, 2);
    log_red!(emu, "ws2_32!WSAGetQOSByName sock:0x{:x}", sock);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}
