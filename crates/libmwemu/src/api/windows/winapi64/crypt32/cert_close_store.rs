use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CertCloseStore(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_store = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg(emu, 1));

    log_red!(
        emu,
        "crypt32!CertCloseStore handle: 0x{:x} flags: 0x{:x}",
        h_store,
        flags
    );

    helper::handler_close(h_store);
    call.set_return(emu, 1);
}
