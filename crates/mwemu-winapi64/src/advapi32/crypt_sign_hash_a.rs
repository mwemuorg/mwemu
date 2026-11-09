use crate::api::windows::common::crypto_state;
use crate::emu;

pub(super) fn CryptSignHashA(emu: &mut emu::Emu) {
    let hhash = emu.regs().rcx;
    let _key_spec = emu.regs().rdx as u32;
    let desc_ptr = emu.regs().r8;
    let _flags = emu.regs().r9 as u32;
    let sig_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let sig_len_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let desc = if desc_ptr != 0 {
        emu.maps.read_string(desc_ptr)
    } else {
        String::new()
    };

    let sig_size = 128usize;
    if let Some(signature) = crypto_state::sign_hash(hhash, sig_size) {
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
        log_red!(
            emu,
            "advapi32!CryptSignHashA hash:0x{:x} desc:'{}' siglen:{}",
            hhash,
            desc,
            signature.len()
        );
    } else {
        log_red!(
            emu,
            "advapi32!CryptSignHashA hash:0x{:x} desc:'{}'",
            hhash,
            desc
        );
    }

    emu.regs_mut().rax = 1;
}
