use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn CryptReleaseContext(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hndl = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg(emu, 1));

    log_red!(emu, "advapi32!CryptReleaseContext");

    helper::handler_close(hndl);

    call.cleanup_args(emu, 2);
    call.set_return(emu, 1);
}
