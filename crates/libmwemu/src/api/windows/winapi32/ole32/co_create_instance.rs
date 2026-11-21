use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub(super) fn CoCreateInstance(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ppv = api_try!(emu, call.arg_u32(emu, 4)) as u64;
    log_red!(emu, "ole32!CoCreateInstance ppv: 0x{:x}", ppv);
    if ppv != 0 {
        emu.maps.write_dword(ppv, 0);
    }
    call.set_return(emu, REGDB_E_CLASSNOTREG);
    call.cleanup_args(emu, 5);
}
