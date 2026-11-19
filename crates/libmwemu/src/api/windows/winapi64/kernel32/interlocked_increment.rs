use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn InterlockedIncrement(emu: &mut emu::Emu, addr: u64) {
    let call = WinApi(Stdcall);
    let addr = api_try!(emu, call.arg(emu, 0));
    let val = emu.maps.read_dword(addr).unwrap_or(0).wrapping_add(1);
    emu.maps.write_dword(addr, val);
    log_red!(emu, "kernel32!InterlockedIncrement 0x{:x} => {}", addr, val);
    call.set_return(emu, val as u64);
}
