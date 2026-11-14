use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptMsgOpenToEncode(emu: &mut emu::Emu) {
    let dw_msg_encoding = WinApiAbi::arg(emu, 0);
    let dw_flags = WinApiAbi::arg(emu, 1);
    let dw_msg_type = WinApiAbi::arg(emu, 2);
    let pv_msg_encode_info = WinApiAbi::arg(emu, 3);
    let psz_inner_content_obj_id = WinApiAbi::arg(emu, 4);
    log_red!(emu, "crypt32!CryptMsgOpenToEncode 0x{:x}", dw_msg_encoding);
    let hndl = helper::handler_create("CryptMsgOpenToEncode");
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 6);
}
