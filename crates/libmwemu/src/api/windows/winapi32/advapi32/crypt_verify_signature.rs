use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn CryptVerifySignatureA(emu: &mut emu::Emu) {
    crypt_verify_signature(emu, "CryptVerifySignatureA", false);
}

fn crypt_verify_signature(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hhash = WinApiAbi::arg(emu, 0);
    let sig_ptr = WinApiAbi::arg(emu, 1);
    let sig_len = WinApiAbi::arg(emu, 2) as u32;
    let hpub_key = WinApiAbi::arg(emu, 3);
    let desc_ptr = WinApiAbi::arg(emu, 4);
    let _flags = WinApiAbi::arg(emu, 5) as u32;
    let desc = if desc_ptr != 0 {
        strings::read_text(emu, desc_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "advapi32!{api} hash:0x{:x} sig:0x{:x} len:{} key:0x{:x} desc:'{}'",
        hhash,
        sig_ptr,
        sig_len,
        hpub_key,
        desc
    );
    for _ in 0..6u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn CryptVerifySignatureW(emu: &mut emu::Emu) {
    crypt_verify_signature(emu, "CryptVerifySignatureW", true);
}
