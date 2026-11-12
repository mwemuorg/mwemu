use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub(super) fn CryptSignHashA(emu: &mut emu::Emu) {
    crypt_sign_hash(emu, "CryptSignHashA", false);
}

fn crypt_sign_hash(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hhash = emu.regs().rcx;
    let _key_spec = emu.regs().rdx as u32;
    let desc_ptr = emu.regs().r8;
    let _flags = emu.regs().r9 as u32;
    let sig_ptr = WinApiAbi::arg(emu, 4);
    let sig_len_ptr = WinApiAbi::arg(emu, 5);
    let desc = if desc_ptr != 0 {
        strings::read_text(emu, desc_ptr, wide)
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
            "advapi32!{api} hash:0x{:x} desc:'{}' siglen:{}",
            hhash,
            desc,
            signature.len()
        );
    } else {
        log_red!(emu, "advapi32!{api} hash:0x{:x} desc:'{}'", hhash, desc);
    }

    emu.regs_mut().rax = 1;
}

pub(super) fn CryptSignHashW(emu: &mut emu::Emu) {
    crypt_sign_hash(emu, "CryptSignHashW", true);
}
