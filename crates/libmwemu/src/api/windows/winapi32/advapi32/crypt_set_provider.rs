use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn CryptSetProviderA(emu: &mut emu::Emu) {
    crypt_set_provider(emu, "CryptSetProviderA", false);
}

fn crypt_set_provider(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let name_ptr = api_try!(emu, call.arg(emu, 0));
    let prov_type = api_try!(emu, call.arg_u32(emu, 1));
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(emu, "advapi32!{api} '{}' type:{}", name, prov_type);
    call.cleanup_args(emu, 2);
    call.set_return(emu, 1);
}

pub fn CryptSetProviderW(emu: &mut emu::Emu) {
    crypt_set_provider(emu, "CryptSetProviderW", true);
}
