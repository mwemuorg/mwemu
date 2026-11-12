use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptUIDlgViewContext(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0);
    log_red!(emu, "crypt32!CryptUIDlgViewContext arg1:0x{:x}", arg1);
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 6);
}
