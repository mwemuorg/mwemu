use crate::emu;
use crate::winapi::common::strings;

pub fn CryptSetProviderA(emu: &mut emu::Emu) {
    crypt_set_provider(emu, "CryptSetProviderA", false);
}

fn crypt_set_provider(emu: &mut emu::Emu, api: &str, wide: bool) {
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let prov_type = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(emu, "advapi32!{api} '{}' type:{}", name, prov_type);
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn CryptSetProviderW(emu: &mut emu::Emu) {
    crypt_set_provider(emu, "CryptSetProviderW", true);
}
