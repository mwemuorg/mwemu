use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn PFXImportCertStore(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _p_pfx = api_try!(emu, call.arg(emu, 0));
    let _sz_password = api_try!(emu, call.arg(emu, 1));
    let _dw_flags = api_try!(emu, call.arg(emu, 2));
    let handle = helper::handler_create("pfx_cert_store");
    log_red!(emu, "crypt32!PFXImportCertStore => 0x{:x}", handle);
    emu.set_last_error(0);
    call.set_return(emu, handle);
}
