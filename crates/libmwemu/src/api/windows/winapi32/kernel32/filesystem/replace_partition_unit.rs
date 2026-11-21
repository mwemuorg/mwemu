use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn ReplacePartitionUnit(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _TargetPartition = api_try!(emu, call.arg_u32(emu, 0));
    let _SparePartition = api_try!(emu, call.arg_u32(emu, 1));
    let _Flags = api_try!(emu, call.arg_u32(emu, 2));

    log_red!(emu, "kernel32!ReplacePartitionUnit");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}
