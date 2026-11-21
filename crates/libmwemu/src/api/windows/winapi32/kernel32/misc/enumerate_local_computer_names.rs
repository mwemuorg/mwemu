use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn EnumerateLocalComputerNamesA(emu: &mut emu::Emu) {
    enumerate_local_computer_names(emu, "EnumerateLocalComputerNamesA", false);
}

pub fn EnumerateLocalComputerNamesW(emu: &mut emu::Emu) {
    enumerate_local_computer_names(emu, "EnumerateLocalComputerNamesW", true);
}

fn enumerate_local_computer_names(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let _a1 = api_try!(emu, call.arg_u32(emu, 0));
    if wide {
        let _lpBuffer = api_try!(emu, call.arg_u32(emu, 1));
        let _a3 = api_try!(emu, call.arg_u32(emu, 2));
        log_red!(emu, "kernel32!{api}");
    } else {
        let lpMultiByteStr = api_try!(emu, call.arg_u32(emu, 1));
        let _a3 = api_try!(emu, call.arg_u32(emu, 2));
        let lpMultiByteStr = strings::read_text(emu, lpMultiByteStr as u64, wide);
        log_red!(emu, "kernel32!{api} {}", lpMultiByteStr);
    }

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}
