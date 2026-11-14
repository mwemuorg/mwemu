use crate::emu;
use crate::winapi::common::strings;

pub fn CryptSetProviderA(emu: &mut emu::Emu) {
    crypt_set_provider(emu, "CryptSetProviderA", false);
}

fn crypt_set_provider(emu: &mut emu::Emu, api: &str, wide: bool) {
    let name_ptr = emu.regs().rcx;
    let prov_type = emu.regs().rdx;
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(emu, "advapi32!{api} '{}' type:{}", name, prov_type);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn CryptSetProviderW(emu: &mut emu::Emu) {
    crypt_set_provider(emu, "CryptSetProviderW", true);
}
