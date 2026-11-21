use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateEventA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ev_attr_ptr = api_try!(emu, call.arg(emu, 0));
    let bManualReset = api_try!(emu, call.arg_u32(emu, 1));
    let bInitialState = api_try!(emu, call.arg_u32(emu, 2));
    let name_ptr = api_try!(emu, call.arg(emu, 3));

    let name = emu.maps.read_string(name_ptr);

    log_red!(emu, "kernel32!CreateEventA `{}`", name);

    call.cleanup_args(emu, 4);
    call.set_return(emu, 1);
}

pub fn CreateEventW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 4);
    log_red!(emu, "kernel32!CreateEventW");
    call.set_return(emu, helper::handler_create("CreateEventW"));
}
