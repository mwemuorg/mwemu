use crate::emu;

pub fn CryptSignMessageWithKey(emu: &mut emu::Emu) {
    let p_sign_para = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let pb_to_be_signed = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let cb_to_be_signed = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let ppb_signed_blob = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    log_red!(emu, "crypt32!CryptSignMessageWithKey 0x{:x}", p_sign_para);
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
