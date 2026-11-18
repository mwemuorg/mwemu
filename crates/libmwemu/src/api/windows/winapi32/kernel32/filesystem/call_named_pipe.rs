use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn CallNamedPipeA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lpNamedPipeName = api_try!(emu, call.arg_u32(emu, 0));
    let _lpInBuffer = api_try!(emu, call.arg_u32(emu, 1));
    let _nInBufferSize = api_try!(emu, call.arg_u32(emu, 2));
    let _lpOutBuffer = api_try!(emu, call.arg_u32(emu, 3));
    let _nOutBufferSize = api_try!(emu, call.arg_u32(emu, 4));
    let _lpBytesRead = api_try!(emu, call.arg_u32(emu, 5));
    let _nTimeOut = api_try!(emu, call.arg_u32(emu, 6));

    let lpNamedPipeName = emu.maps.read_string(lpNamedPipeName as u64);

    log_red!(emu, "kernel32!CallNamedPipeA {}", lpNamedPipeName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 7);
}

pub fn CallNamedPipeW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let p2 = api_try!(emu, call.arg(emu, 2));
    let p3 = api_try!(emu, call.arg(emu, 3));
    let p4 = api_try!(emu, call.arg(emu, 4));
    let p5 = api_try!(emu, call.arg(emu, 5));
    let p6 = api_try!(emu, call.arg(emu, 6));
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernel32!CallNamedPipeW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
    call.cleanup_args(emu, 7);
    call.set_return(emu, 1);
}
