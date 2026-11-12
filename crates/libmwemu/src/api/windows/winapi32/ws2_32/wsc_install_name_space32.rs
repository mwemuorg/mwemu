use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSCInstallNameSpace32(emu: &mut emu::Emu) {
    let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!WSCInstallNameSpace32 ptr:0x{:x}", err_ptr);
    let handle = helper::handler_create("WSCInstallNameSpace32");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 5);
}
