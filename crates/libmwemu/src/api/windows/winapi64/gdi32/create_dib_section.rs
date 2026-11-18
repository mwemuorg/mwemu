use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateDIBSection(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let pbmi = api_try!(emu, call.arg(emu, 1));
    let usage = api_try!(emu, call.arg(emu, 2));
    let ppv_bits = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "gdi32!CreateDIBSection hdc:0x{:x} bmi:0x{:x} usage:{} bits_out:0x{:x}",
        hdc,
        pbmi,
        usage,
        ppv_bits
    );
    call.set_return(emu, helper::handler_create("gdi32://bitmap"));
}
