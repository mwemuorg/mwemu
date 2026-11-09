use crate::emu;

pub fn CryptDecryptMessage(emu: &mut emu::Emu) {
    let p_decrypt_para = emu.regs().rcx;
    let pb_encrypted_blob = emu.regs().rdx;
    let cb_encrypted_blob = emu.regs().r8 as u32;
    let pb_decrypted = emu.regs().r9;
    let pcb_decrypted = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let pp_xchg_cert = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    let blob_data = if pb_encrypted_blob != 0 && cb_encrypted_blob > 0 {
        let len = (cb_encrypted_blob as usize).min(256);
        emu.maps.read_bytes(pb_encrypted_blob, len).to_vec()
    } else {
        Vec::new()
    };

    if pb_decrypted != 0 && !blob_data.is_empty() {
        emu.maps.write_bytes(pb_decrypted, &blob_data);
    }
    if pcb_decrypted != 0 {
        emu.maps.write_dword(pcb_decrypted, blob_data.len() as u32);
    }

    log_red!(
        emu,
        "crypt32!CryptDecryptMessage blob_sz:{} out:0x{:x}",
        cb_encrypted_blob,
        pb_decrypted
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
