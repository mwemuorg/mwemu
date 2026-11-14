use crate::emu;
use crate::winapi::helper;

pub fn WinHttpCloseHandle(emu: &mut emu::Emu) {
    let h_internet = emu.regs().rcx;

    log_red!(emu, "winhttp!WinHttpCloseHandle handle: 0x{:x}", h_internet);

    helper::handler_close(h_internet);
    emu.regs_mut().rax = 1;
}
