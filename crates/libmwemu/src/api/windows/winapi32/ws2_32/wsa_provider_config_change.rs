use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSAProviderConfigChange(emu: &mut emu::Emu) {
    let handle_ptr = WinApiAbi::arg(emu, 0);
    let overlapped_ptr = WinApiAbi::arg(emu, 1);
    log_red!(emu, "ws2_32!WSAProviderConfigChange");
    if handle_ptr != 0 {
        let handle = helper::handler_create("wsa_provider_change");
        emu.maps.write_dword(handle_ptr, handle as u32);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}
