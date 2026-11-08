use crate::emu;

pub fn CryptDecryptAndVerifyMessageSignature(emu: &mut emu::Emu) {
    let p_decrypt_para = emu.regs().rcx;
    let p_verify_para = emu.regs().rdx;
    let dw_signer_index = emu.regs().r8 as u32;
    let pb_encrypted_blob = emu.regs().r9;
    let cb_encrypted_blob = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0) as u32;
    let ppb_decrypted = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let pcb_decrypted = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let pp_xchg_cert = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
    let pp_signer_cert = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);

    let blob_data = if pb_encrypted_blob != 0 && cb_encrypted_blob > 0 {
        let len = (cb_encrypted_blob as usize).min(256);
        emu.maps.read_bytes(pb_encrypted_blob, len).to_vec()
    } else {
        Vec::new()
    };

    if ppb_decrypted != 0 && !blob_data.is_empty() {
        let out_ptr = emu.maps.alloc(blob_data.len() as u64).expect("alloc");
        emu.maps.write_bytes(out_ptr, &blob_data);
        emu.maps.write_qword(ppb_decrypted, out_ptr);
        if pcb_decrypted != 0 {
            emu.maps.write_dword(pcb_decrypted, blob_data.len() as u32);
        }
    }

    log_red!(
        emu,
        "crypt32!CryptDecryptAndVerifyMessageSignature blob_sz:{}",
        cb_encrypted_blob
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
