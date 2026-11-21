use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptReleaseContext(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_prov = api_try!(emu, call.arg(emu, 0));
    let dw_flags = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(
        emu,
        "crypt32!CryptReleaseContext prov: 0x{:x} flags: 0x{:x}",
        h_prov,
        dw_flags
    );
    if h_prov != 0 {
        helper::handler_close(h_prov);
    }
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}
