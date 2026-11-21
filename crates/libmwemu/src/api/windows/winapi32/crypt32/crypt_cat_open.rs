use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptCATOpen(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pwsz_file_name = api_try!(emu, call.arg(emu, 0));
    let fdw_open_flags = api_try!(emu, call.arg(emu, 1));
    let h_prov = api_try!(emu, call.arg(emu, 2));
    let dw_public_version = api_try!(emu, call.arg(emu, 3));
    let dw_encoded_msg_type = api_try!(emu, call.arg(emu, 4));
    log_red!(emu, "crypt32!CryptCATOpen 0x{:x}", pwsz_file_name);
    let hndl = helper::handler_create("CryptCATOpen");
    call.set_return(emu, hndl);
    call.cleanup_args(emu, 5);
}
