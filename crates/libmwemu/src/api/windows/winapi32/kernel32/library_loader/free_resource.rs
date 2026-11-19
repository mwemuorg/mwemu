use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn FreeResource(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hResData = api_try!(emu, call.arg_u32(emu, 0)) as u64;

    call.cleanup_args(emu, 1);

    log_red!(emu, "** {} kernel32!FreeResource {:x}", emu.pos, hResData);
    helper::handler_close(hResData);

    call.set_return(emu, 1);
}
