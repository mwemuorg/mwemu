use crate::emu;
use crate::winapi::helper;

pub fn CryptMsgOpenToDecode(emu: &mut emu::Emu) {
    let dw_msg_encoding = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let dw_msg_type = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let h_crypt_prov = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let p_recipient_info = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    log_red!(emu, "crypt32!CryptMsgOpenToDecode 0x{:x}", dw_msg_encoding);
    let hndl = helper::handler_create("CryptMsgOpenToDecode");
    emu.regs_mut().rax = hndl;
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
