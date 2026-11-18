use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn SetWorldTransform(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let xform = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "gdi32!SetWorldTransform hdc:0x{:x} xform:0x{:x}",
        hdc,
        xform
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
