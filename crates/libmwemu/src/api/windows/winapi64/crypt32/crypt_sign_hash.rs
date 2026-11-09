use crate::api::windows::common::crypto_state;
use crate::emu;

pub fn CryptSignHash(emu: &mut emu::Emu) {
    let h_hash = emu.regs().rcx;
    let key_spec = emu.regs().rdx as u32;
    let _desc_ptr = emu.regs().r8;
    let flags = emu.regs().r9 as u32;
    let sig_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let sig_len_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    log_red!(
        emu,
        "crypt32!CryptSignHash hash:0x{:x} spec:{} flags:0x{:x}",
        h_hash,
        key_spec,
        flags
    );
    let sig_size = 128usize;
    if let Some(signature) = crypto_state::sign_hash(h_hash, sig_size) {
        if sig_ptr == 0 {
            if sig_len_ptr != 0 {
                emu.maps.write_dword(sig_len_ptr, signature.len() as u32);
            }
        } else {
            let write_len = if sig_len_ptr != 0 {
                let buf_sz = emu.maps.read_dword(sig_len_ptr).unwrap_or(0) as usize;
                signature.len().min(buf_sz)
            } else {
                signature.len()
            };
            emu.maps.write_bytes(sig_ptr, &signature[..write_len]);
            if sig_len_ptr != 0 {
                emu.maps.write_dword(sig_len_ptr, write_len as u32);
            }
        }
    } else if sig_ptr == 0 {
        if sig_len_ptr != 0 {
            emu.maps.write_dword(sig_len_ptr, sig_size as u32);
        }
    } else {
        for i in 0..sig_size as u64 {
            emu.maps.write_byte(sig_ptr + i, 0xff);
        }
        if sig_len_ptr != 0 {
            emu.maps.write_dword(sig_len_ptr, sig_size as u32);
        }
    }
    emu.regs_mut().rax = 1;
}
