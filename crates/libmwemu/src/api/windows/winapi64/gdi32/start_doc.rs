use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn StartDoc(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let docinfo = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "gdi32!StartDoc hdc:0x{:x} docinfo:0x{:x}",
        hdc,
        docinfo
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, 1); // job id
}
