use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn WinHttpSendRequest(emu: &mut emu::Emu) {
    let request = arg(emu, 0);
    log_red!(emu, "winhttp!WinHttpSendRequest request: 0x{:x}", request);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 7);
}
