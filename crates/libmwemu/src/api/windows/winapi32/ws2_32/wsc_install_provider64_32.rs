use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSCInstallProvider64_32(emu: &mut emu::Emu) {
    let err_ptr = WinApiAbi::arg(emu, 0);
    log_red!(emu, "ws2_32!WSCInstallProvider64_32 ptr:0x{:x}", err_ptr);
    let handle = helper::handler_create("WSCInstallProvider64_32");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 5);
}
