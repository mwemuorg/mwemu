use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptRetrieveObjectByUrl(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let psz_url = api_try!(emu, call.arg(emu, 0));
    let psz_object_oid = api_try!(emu, call.arg(emu, 1));
    let dw_retrieve_flags = api_try!(emu, call.arg(emu, 2));
    let dw_timeout = api_try!(emu, call.arg(emu, 3));
    let ppv_object = api_try!(emu, call.arg(emu, 4));
    let h_async_retrieve = api_try!(emu, call.arg(emu, 5));
    let p_credentials = api_try!(emu, call.arg(emu, 6));
    log_red!(emu, "crypt32!CryptRetrieveObjectByUrl 0x{:x}", psz_url);
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 7);
}
