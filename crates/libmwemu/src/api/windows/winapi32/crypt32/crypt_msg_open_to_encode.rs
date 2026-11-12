use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptMsgOpenToEncode(emu: &mut emu::Emu) {
    let dw_msg_encoding = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let dw_msg_type = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let pv_msg_encode_info = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let psz_inner_content_obj_id =
        emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    log_red!(emu, "crypt32!CryptMsgOpenToEncode 0x{:x}", dw_msg_encoding);
    let hndl = helper::handler_create("CryptMsgOpenToEncode");
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 6);
}
