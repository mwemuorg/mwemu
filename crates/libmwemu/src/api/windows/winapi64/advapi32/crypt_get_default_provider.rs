use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn CryptGetDefaultProviderA(emu: &mut emu::Emu) {
    crypt_get_default_provider(emu, "CryptGetDefaultProviderA", false);
}

pub fn CryptGetDefaultProviderW(emu: &mut emu::Emu) {
    crypt_get_default_provider(emu, "CryptGetDefaultProviderW", true);
}

fn crypt_get_default_provider(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let _prov_type = api_try!(emu, call.arg(emu, 0));
    let _reserved = api_try!(emu, call.arg(emu, 1));
    let _flags = api_try!(emu, call.arg(emu, 2));
    let name_ptr = api_try!(emu, call.arg(emu, 3));
    let name_len_ptr = api_try!(emu, call.arg(emu, 4));
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
    call.set_return(emu, 1);
}
