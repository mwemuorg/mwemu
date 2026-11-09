use crate::emu;
use crate::winapi::helper;

pub fn WSAProviderConfigChange(emu: &mut emu::Emu) {
    let handle_ptr = emu.regs().rcx;
    let overlapped_ptr = emu.regs().rdx;
    let completion_ptr = emu.regs().r8;
    log_red!(emu, "ws2_32!WSAProviderConfigChange");
    if handle_ptr != 0 {
        let handle = helper::handler_create("wsa_provider_change");
        emu.maps.write_qword(handle_ptr, handle);
    }
    emu.regs_mut().rax = 0;
}
