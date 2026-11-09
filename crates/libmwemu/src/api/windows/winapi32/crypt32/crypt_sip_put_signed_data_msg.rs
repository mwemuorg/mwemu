use crate::emu;

pub fn CryptSIPPutSignedDataMsg(emu: &mut emu::Emu) {
    let p_subject_info = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let dw_encoding_type = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let pdw_index = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let cb_signed_data_msg = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let pb_signed_data_msg = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptSIPPutSignedDataMsg 0x{:x}",
        p_subject_info
    );
    let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
