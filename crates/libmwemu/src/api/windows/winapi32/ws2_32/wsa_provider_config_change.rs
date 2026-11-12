use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSAProviderConfigChange(emu: &mut emu::Emu) {
    let handle_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let overlapped_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!WSAProviderConfigChange");
    if handle_ptr != 0 {
        let handle = helper::handler_create("wsa_provider_change");
        emu.maps.write_dword(handle_ptr, handle as u32);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}
