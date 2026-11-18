use super::EOF;
use super::stream_handle;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn fflush(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let file = api_try!(emu, call.arg(emu, 0));
    let ok = file == 0 || stream_handle(file).is_some();
    log_red!(emu, "wincrt!fflush 0x{:x}", file);
    call.set_return(emu, if ok { 0 } else { EOF });
}
