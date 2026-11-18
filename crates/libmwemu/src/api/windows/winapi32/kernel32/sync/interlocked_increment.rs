use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn InterlockedIncrement(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let addend = api_try!(emu, call.arg_u32(emu, 0));

    let prev = emu.maps.read_dword(addend as u64).unwrap_or(0);

    emu.maps.write_dword(addend as u64, prev + 1);

    log_red!(
        emu,
        "kernel32!InterlockedIncrement 0x{:x} {}->{}",
        addend,
        prev,
        prev + 1
    );

    call.cleanup_args(emu, 1);
    call.set_return(emu, prev as u64 + 1);
}
