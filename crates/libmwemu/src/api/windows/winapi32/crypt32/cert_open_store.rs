use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CertOpenStore(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let store_provider = api_try!(emu, call.arg_u32(emu, 0));
    let encoding_type = api_try!(emu, call.arg_u32(emu, 1));
    let _hcrypt_prov = api_try!(emu, call.arg_u32(emu, 2));
    let flags = api_try!(emu, call.arg_u32(emu, 3));

    log_red!(
        emu,
        "crypt32!CertOpenStore provider: 0x{:x} enc: 0x{:x} flags: 0x{:x}",
        store_provider,
        encoding_type,
        flags
    );

    let handle = helper::handler_create("cert_store");
    call.set_return(emu, handle);
    call.cleanup_args(emu, 5);
}
