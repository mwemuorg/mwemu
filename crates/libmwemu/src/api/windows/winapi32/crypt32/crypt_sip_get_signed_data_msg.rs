use crate::emu;

pub fn CryptSIPGetSignedDataMsg(emu: &mut emu::Emu) {
    let p_subject_info = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let pdw_encoding_type = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let dw_index = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let pcb_signed_data_msg = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let pb_signed_data_msg = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptSIPGetSignedDataMsg 0x{:x}",
        p_subject_info
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
