use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn InterlockedExchange(emu: &mut emu::Emu, addr: u64) {
    let call = WinApi(Stdcall);
    let addr = api_try!(emu, call.arg(emu, 0));
    let new_val = api_try!(emu, call.arg_u32(emu, 1));
    let old = emu.maps.read_dword(addr).unwrap_or(0);
    emu.maps.write_dword(addr, new_val);
    log_red!(
        emu,
        "kernel32!InterlockedExchange 0x{:x} {} => {}",
        addr,
        new_val,
        old
    );
    call.set_return(emu, old as u64);
}
