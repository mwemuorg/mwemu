use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn IsWow64Process(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let out_ptr = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    log_red!(emu, "kernel32!IsWow64Process out: 0x{:x}", out_ptr);
    // A 32-bit process on the 64-bit Windows the emulator models is WOW64.
    emu.maps.write_dword(out_ptr, 1);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}
