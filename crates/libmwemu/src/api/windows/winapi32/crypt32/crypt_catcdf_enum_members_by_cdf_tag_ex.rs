use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptCATCDFEnumMembersByCDFTagEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p_cdf = api_try!(emu, call.arg(emu, 0));
    let pwsz_prev_tag = api_try!(emu, call.arg(emu, 1));
    let pfn_parse_error = api_try!(emu, call.arg(emu, 2));
    let pp_member = api_try!(emu, call.arg(emu, 3));
    let f_continue_on_error = api_try!(emu, call.arg(emu, 4));
    log_red!(
        emu,
        "crypt32!CryptCATCDFEnumMembersByCDFTagEx 0x{:x}",
        p_cdf
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 6);
}
