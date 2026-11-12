use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptEnumOIDInfo(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0);
    log_red!(emu, "crypt32!CryptEnumOIDInfo arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
