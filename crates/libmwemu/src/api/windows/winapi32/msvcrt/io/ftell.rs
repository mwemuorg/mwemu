use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn ftell(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let fp = api_try!(emu, call.arg(emu, 0));
    let pos = helper::handler_tell(fp);
    log_red!(emu, "msvcrt!ftell fp:0x{:x} => {}", fp, pos);
    call.set_return(emu, pos as u64);
}
