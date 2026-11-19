use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn WriteConsoleInputVDMA(emu: &mut emu::Emu) {
    write_console_input_vdm(emu, "WriteConsoleInputVDMA");
}

fn write_console_input_vdm(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let _a1 = api_try!(emu, call.arg_u32(emu, 0));
    let _a2 = api_try!(emu, call.arg_u32(emu, 1));
    let _a3 = api_try!(emu, call.arg_u32(emu, 2));
    let _a4 = api_try!(emu, call.arg_u32(emu, 3));

    log_red!(emu, "kernel32!{api}");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 4);
}

pub fn WriteConsoleInputVDMW(emu: &mut emu::Emu) {
    write_console_input_vdm(emu, "WriteConsoleInputVDMW");
}
