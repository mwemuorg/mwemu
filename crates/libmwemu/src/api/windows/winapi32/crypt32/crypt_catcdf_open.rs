use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptCATCDFOpen(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pwsz_file_path = api_try!(emu, call.arg(emu, 0));
    let pfn_parse_error = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "crypt32!CryptCATCDFOpen pwsz_file_path:0x{:x} pfn_parse_error:0x{:x}",
        pwsz_file_path,
        pfn_parse_error
    );
    let hndl = helper::handler_create("CryptCATCDFOpen");
    call.set_return(emu, hndl);
    call.cleanup_args(emu, 2);
}
