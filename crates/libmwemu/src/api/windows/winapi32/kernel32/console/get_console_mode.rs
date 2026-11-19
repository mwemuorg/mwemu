use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetConsoleMode(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg_u32(emu, 0));
    let mode_ptr = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    log_red!(emu, "kernel32!GetConsoleMode hndl: 0x{:x}", handle);
    emu.maps.write_dword(mode_ptr, 3); // ENABLE_PROCESSED_INPUT | ENABLE_LINE_INPUT
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}
