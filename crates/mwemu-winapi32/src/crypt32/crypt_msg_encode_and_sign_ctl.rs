use crate::emu;

pub fn CryptMsgEncodeAndSignCTL(emu: &mut emu::Emu) {
    let dw_msg_encoding = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p_ctl_info = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let p_sign_info = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptMsgEncodeAndSignCTL 0x{:x}",
        dw_msg_encoding
    );
    let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
