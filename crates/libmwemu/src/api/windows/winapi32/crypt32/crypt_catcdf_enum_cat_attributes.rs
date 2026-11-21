use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptCATCDFEnumCatAttributes(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p_cdf = api_try!(emu, call.arg(emu, 0));
    let p_prev_attr = api_try!(emu, call.arg(emu, 1));
    let pfn_parse_error = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "crypt32!CryptCATCDFEnumCatAttributes p_cdf:0x{:x} p_prev_attr:0x{:x} pfn_parse_error:0x{:x}",
        p_cdf,
        p_prev_attr,
        pfn_parse_error
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 3);
}
