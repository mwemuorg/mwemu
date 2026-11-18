use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::exception::raise;
use crate::winapi::policy::api_try;
use crate::windows::constants::exceptions::EXCEPTION_MAXIMUM_PARAMETERS;

pub fn RaiseException(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let code = api_try!(emu, call.arg_u32(emu, 0));
    let flags = api_try!(emu, call.arg_u32(emu, 1));
    let num_args = api_try!(emu, call.arg_u32(emu, 2));
    let args_ptr = api_try!(emu, call.arg(emu, 3));

    log_red!(
        emu,
        "kernel32!RaiseException code:0x{:08x} flags:0x{:x} nArgs:{} pArgs:0x{:x}",
        code,
        flags,
        num_args,
        args_ptr
    );

    let count = (num_args as usize).min(EXCEPTION_MAXIMUM_PARAMETERS);
    let params: Vec<u64> = (0..count as u64)
        .map(|i| emu.maps.read_qword(args_ptr + i * 8).unwrap_or(0))
        .collect();

    if raise::raise64(emu, code, flags, &params) {
        return;
    }

    call.set_return(emu, 0);
}
