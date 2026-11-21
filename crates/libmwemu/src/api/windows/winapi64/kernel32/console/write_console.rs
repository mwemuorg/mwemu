use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn WriteConsoleA(emu: &mut Emu) {
    invoke(emu, false, "WriteConsoleA");
}

pub fn WriteConsoleW(emu: &mut Emu) {
    invoke(emu, true, "WriteConsoleW");
}

fn invoke(emu: &mut Emu, wide: bool, api: &str) {
    let call = WinApi(Stdcall);
    let argument_0 = api_try!(emu, call.arg(emu, 0));
    let input = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg_u32(emu, 2));
    let written = api_try!(emu, call.arg(emu, 3));
    let reserved = api_try!(emu, call.arg(emu, 4));
    let size = count as usize
        * if wide {
            size_of::<u16>()
        } else {
            size_of::<u8>()
        };
    log_red!(
        emu,
        "kernel32!{} handle:0x{:x} input:0x{:x} count:{}",
        api,
        argument_0,
        input,
        count
    );
    let error = if reserved != 0 {
        ERROR_INVALID_PARAMETER
    } else if !emu.maps.can_access_range(input, size, Permission::READ)
        || (written != 0 && !emu.maps.write_dword(written, count))
    {
        ERROR_NOACCESS
    } else {
        ERROR_SUCCESS
    };
    if error != ERROR_SUCCESS {
        emu.set_last_error(error);
    }
    call.set_return(emu, (error == ERROR_SUCCESS) as u64);
}
