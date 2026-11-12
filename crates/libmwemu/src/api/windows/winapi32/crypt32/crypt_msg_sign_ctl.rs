use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptMsgSignCTL(emu: &mut emu::Emu) {
    let dw_msg_encoding = WinApiAbi::arg(emu, 0);
    let pb_ctl_content = WinApiAbi::arg(emu, 1);
    let cb_ctl_content = WinApiAbi::arg(emu, 2);
    let p_sign_info = WinApiAbi::arg(emu, 3);
    log_red!(emu, "crypt32!CryptMsgSignCTL 0x{:x}", dw_msg_encoding);
    let val_ptr = WinApiAbi::arg(emu, 1);
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 7);
}
