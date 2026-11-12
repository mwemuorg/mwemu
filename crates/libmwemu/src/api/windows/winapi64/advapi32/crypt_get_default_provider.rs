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
    let _prov_type = emu.regs().rcx;
    let _reserved = emu.regs().rdx;
    let _flags = emu.regs().r8;
    let name_ptr = emu.regs().r9;
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
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
