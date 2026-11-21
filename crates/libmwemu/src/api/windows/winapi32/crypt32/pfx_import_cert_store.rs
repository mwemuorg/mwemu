use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn PFXImportCertStore(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pfx_ptr = api_try!(emu, call.arg(emu, 0));
    let _password_ptr = api_try!(emu, call.arg_u32(emu, 1));
    let flags = api_try!(emu, call.arg_u32(emu, 2));
    call.cleanup_args(emu, 3);
    log_red!(
        emu,
        "crypt32!PFXImportCertStore pfx: 0x{:x} flags: 0x{:x}",
        pfx_ptr,
        flags
    );
    call.set_return(emu, helper::handler_create("certstore:pfx"));
}
