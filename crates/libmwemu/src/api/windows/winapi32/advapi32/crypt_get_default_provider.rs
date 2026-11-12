use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn CryptGetDefaultProviderA(emu: &mut emu::Emu) {
    crypt_get_default_provider(emu, "CryptGetDefaultProviderA", false);
}

pub fn CryptGetDefaultProviderW(emu: &mut emu::Emu) {
    crypt_get_default_provider(emu, "CryptGetDefaultProviderW", true);
}

fn crypt_get_default_provider(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _prov_type = WinApiAbi::arg(emu, 0) as u32;
    let _reserved = WinApiAbi::arg(emu, 1) as u32;
    let _flags = WinApiAbi::arg(emu, 2) as u32;
    let name_ptr = WinApiAbi::arg(emu, 3);
    let name_len_ptr = WinApiAbi::arg(emu, 4);
    let name = "Microsoft Base Cryptographic Provider v1.0";
    if name_ptr != 0 {
        strings::write_string(emu, name_ptr, name, wide);
    }
    if wide {
        if name_len_ptr != 0 {
            emu.maps
                .write_dword(name_len_ptr, (name.len() as u32 + 1) * 2);
        }
    } else {
        if name_len_ptr != 0 {
            emu.maps.write_dword(name_len_ptr, name.len() as u32 + 1);
        }
    }

    log_red!(emu, "advapi32!{api} => '{}'", name);
    for _ in 0..5u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
