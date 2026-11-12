use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSCEnumProtocols(emu: &mut emu::Emu) {
    let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!WSCEnumProtocols ptr:0x{:x}", err_ptr);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}
