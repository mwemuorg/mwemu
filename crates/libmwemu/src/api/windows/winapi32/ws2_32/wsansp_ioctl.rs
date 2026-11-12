use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSANSPIoctl(emu: &mut emu::Emu) {
    let lookup_handle = WinApiAbi::arg(emu, 0);
    let control_code = WinApiAbi::arg(emu, 1) as u32;
    log_red!(
        emu,
        "ws2_32!WSANSPIoctl handle:0x{:x} code:0x{:x}",
        lookup_handle,
        control_code
    );
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 8);
}
