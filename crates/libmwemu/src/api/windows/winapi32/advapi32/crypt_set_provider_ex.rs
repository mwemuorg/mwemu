use crate::emu;
use crate::winapi::common::strings;

pub fn CryptSetProviderExA(emu: &mut emu::Emu) {
    crypt_set_provider_ex(emu, "CryptSetProviderExA", false);
}

fn crypt_set_provider_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let prov_type = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(
        emu,
        "advapi32!{api} '{}' type:{} flags:0x{:x}",
        name,
        prov_type,
        flags
    );
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn CryptSetProviderExW(emu: &mut emu::Emu) {
    crypt_set_provider_ex(emu, "CryptSetProviderExW", true);
}
