use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn PkiInitializeCriticalSection(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let addr = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg(emu, 1));

    log_red!(
        emu,
        "crypt32!PkiInitializeCriticalSection flags: 0x{:x} addr: 0x{:x}",
        flags,
        addr
    );

    call.set_return(emu, 1);
}
