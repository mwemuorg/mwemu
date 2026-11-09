use crate::emu;

pub fn CryptDecryptMessage(emu: &mut emu::Emu) {
    let p_decrypt_para = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let pb_encrypted_blob = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let cb_encrypted_blob = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptDecryptMessage p_decrypt_para:0x{:x} pb_encrypted_blob:0x{:x} cb_encrypted_blob:0x{:x}",
        p_decrypt_para,
        pb_encrypted_blob,
        cb_encrypted_blob
    );
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
