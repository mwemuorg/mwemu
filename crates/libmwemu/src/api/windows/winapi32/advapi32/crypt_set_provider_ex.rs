use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn CryptSetProviderExA(emu: &mut emu::Emu) {
    crypt_set_provider_ex(emu, "CryptSetProviderExA", false);
}

fn crypt_set_provider_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let name_ptr = WinApiAbi::arg(emu, 0);
    let prov_type = WinApiAbi::arg(emu, 1) as u32;
    let _reserved = WinApiAbi::arg(emu, 2) as u32;
    let flags = WinApiAbi::arg(emu, 3) as u32;
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
