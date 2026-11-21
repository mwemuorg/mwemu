use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn CryptSetProviderExA(emu: &mut emu::Emu) {
    crypt_set_provider_ex(emu, "CryptSetProviderExA", false);
}

fn crypt_set_provider_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let name_ptr = api_try!(emu, call.arg(emu, 0));
    let prov_type = api_try!(emu, call.arg(emu, 1));
    let _reserved = api_try!(emu, call.arg(emu, 2));
    let flags = api_try!(emu, call.arg(emu, 3));
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(
        emu,
        "advapi32!{api} '{}' type:{} flags:0x{:x}",
        name,
        prov_type,
        flags
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn CryptSetProviderExW(emu: &mut emu::Emu) {
    crypt_set_provider_ex(emu, "CryptSetProviderExW", true);
}
