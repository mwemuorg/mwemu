use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn CryptVerifySignatureA(emu: &mut emu::Emu) {
    crypt_verify_signature(emu, "CryptVerifySignatureA", false);
}

fn crypt_verify_signature(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hhash = emu.regs().rcx;
    let sig_ptr = emu.regs().rdx;
    let sig_len = emu.regs().r8;
    let hpub_key = emu.regs().r9;
    let desc_ptr = WinApiAbi::arg(emu, 4);
    let _flags = WinApiAbi::arg(emu, 5);
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
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn CryptVerifySignatureW(emu: &mut emu::Emu) {
    crypt_verify_signature(emu, "CryptVerifySignatureW", true);
}
