use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn InterlockedCompareExchange(emu: &mut emu::Emu, addr: u64) {
    let call = WinApi(Stdcall);
    let addr = api_try!(emu, call.arg(emu, 0));
    let exchange = api_try!(emu, call.arg_u32(emu, 1));
    let comparand = api_try!(emu, call.arg_u32(emu, 2));
    let current = emu.maps.read_dword(addr).unwrap_or(0);
    if current == comparand {
        emu.maps.write_dword(addr, exchange);
    }
    log_red!(
        emu,
        "kernel32!InterlockedCompareExchange 0x{:x} exch:{} cmp:{} cur:{}",
        addr,
        exchange,
        comparand,
        current
    );
    call.set_return(emu, current as u64);
}
