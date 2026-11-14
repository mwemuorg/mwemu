use crate::emu;
use crate::winapi::common::strings;

pub fn CryptSetProviderExA(emu: &mut emu::Emu) {
    crypt_set_provider_ex(emu, "CryptSetProviderExA", false);
}

fn crypt_set_provider_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let name_ptr = emu.regs().rcx;
    let prov_type = emu.regs().rdx;
    let _reserved = emu.regs().r8;
    let flags = emu.regs().r9;
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(
        emu,
        "advapi32!{api} '{}' type:{} flags:0x{:x}",
        name,
        prov_type,
        flags
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn CryptSetProviderExW(emu: &mut emu::Emu) {
    crypt_set_provider_ex(emu, "CryptSetProviderExW", true);
}
