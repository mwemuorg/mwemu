use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GetNamedPipeClientComputerNameA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _Pipe = api_try!(emu, call.arg_u32(emu, 0));
    let ClientComputerName = api_try!(emu, call.arg_u32(emu, 1));
    let ClientComputerNameLength = api_try!(emu, call.arg_u32(emu, 2));

    let ClientComputerName = emu.maps.read_string(ClientComputerName as u64);
    let ClientComputerNameLength = emu.maps.read_string(ClientComputerNameLength as u64);

    log_red!(
        emu,
        "kernel32!GetNamedPipeClientComputerNameA {} {}",
        ClientComputerName,
        ClientComputerNameLength
    );

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}

pub fn GetNamedPipeClientComputerNameW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(
        emu,
        "kernel32!GetNamedPipeClientComputerNameW '{}'  0x{:x}",
        s,
        arg2
    );
    call.cleanup_args(emu, 3);
    call.set_return(emu, 1);
}
