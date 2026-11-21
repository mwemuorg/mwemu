use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn CryptEnumProviderTypesA(emu: &mut emu::Emu) {
    crypt_enum_provider_types(emu, "CryptEnumProviderTypesA", false);
}

pub fn CryptEnumProviderTypesW(emu: &mut emu::Emu) {
    crypt_enum_provider_types(emu, "CryptEnumProviderTypesW", true);
}

fn crypt_enum_provider_types(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let index = api_try!(emu, call.arg_u32(emu, 0));
    let _reserved = api_try!(emu, call.arg_u32(emu, 1));
    let _flags = api_try!(emu, call.arg_u32(emu, 2));
    let prov_type_ptr = api_try!(emu, call.arg(emu, 3));
    let name_ptr = api_try!(emu, call.arg(emu, 4));
    let name_len_ptr = api_try!(emu, call.arg(emu, 5));
    log_red!(emu, "advapi32!{api} index:{}", index);
    if wide {
        if index == 0 {
            if prov_type_ptr != 0 {
                emu.maps.write_dword(prov_type_ptr, 1);
            }
            let name = "RSA Full (Signature and Key Exchange)";
            if name_ptr != 0 {
                strings::write_string(emu, name_ptr, name, wide);
            }
            if name_len_ptr != 0 {
                emu.maps
                    .write_dword(name_len_ptr, (name.len() as u32 + 1) * 2);
            }
            call.set_return(emu, 1);
        } else {
            call.set_return(emu, 0);
        }
    } else {
        if index == 0 {
            if prov_type_ptr != 0 {
                emu.maps.write_dword(prov_type_ptr, 1);
            }
            let name = "RSA Full (Signature and Key Exchange)";
            if name_ptr != 0 {
                strings::write_string(emu, name_ptr, name, wide);
            }
            if name_len_ptr != 0 {
                emu.maps.write_dword(name_len_ptr, name.len() as u32 + 1);
            }
            call.set_return(emu, 1);
        } else {
            call.set_return(emu, 0);
        }
    }

    call.cleanup_args(emu, 6);
}
