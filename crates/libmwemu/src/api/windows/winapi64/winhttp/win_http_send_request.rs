use crate::emu;
use crate::winapi::helper;

pub fn WinHttpSendRequest(emu: &mut emu::Emu) {
    let h_request = emu.regs().rcx;

    log_red!(emu, "winhttp!WinHttpSendRequest request: 0x{:x}", h_request);

    if !helper::handler_exist(h_request) {
        log::trace!("\tinvalid request handle");
    }

    emu.regs_mut().rax = 1;
}
