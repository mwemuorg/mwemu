use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn CryptEnumProvidersA(emu: &mut emu::Emu) {
    crypt_enum_providers(emu, "CryptEnumProvidersA", false);
}

pub fn CryptEnumProvidersW(emu: &mut emu::Emu) {
    crypt_enum_providers(emu, "CryptEnumProvidersW", true);
}

fn crypt_enum_providers(emu: &mut emu::Emu, api: &str, wide: bool) {
    let index = WinApiAbi::arg(emu, 0) as u32;
    let _reserved = WinApiAbi::arg(emu, 1) as u32;
    let _flags = WinApiAbi::arg(emu, 2) as u32;
    let prov_type_ptr = WinApiAbi::arg(emu, 3);
    let name_ptr = WinApiAbi::arg(emu, 4);
    let name_len_ptr = WinApiAbi::arg(emu, 5);
    log_red!(emu, "advapi32!{api} index:{}", index);
    if wide {
        if index == 0 {
            if prov_type_ptr != 0 {
                emu.maps.write_dword(prov_type_ptr, 1);
            }
            let name = "Microsoft Base Cryptographic Provider v1.0";
            if name_ptr != 0 {
                strings::write_string(emu, name_ptr, name, wide);
            }
            if name_len_ptr != 0 {
                emu.maps
                    .write_dword(name_len_ptr, (name.len() as u32 + 1) * 2);
            }
            emu.regs_mut().rax = 1;
        } else {
            emu.regs_mut().rax = 0;
        }
    } else {
        if index == 0 {
            if prov_type_ptr != 0 {
                emu.maps.write_dword(prov_type_ptr, 1);
            }
            let name = "Microsoft Base Cryptographic Provider v1.0";
            if name_ptr != 0 {
                strings::write_string(emu, name_ptr, name, wide);
            }
            if name_len_ptr != 0 {
                emu.maps.write_dword(name_len_ptr, name.len() as u32 + 1);
            }
            emu.regs_mut().rax = 1;
        } else {
            emu.regs_mut().rax = 0;
        }
    }

    for _ in 0..6u32 {
        emu.stack_pop32(false);
    }
}
