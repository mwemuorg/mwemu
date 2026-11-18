use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn localtime(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let time_ptr = api_try!(emu, call.arg(emu, 0));
    let tm_addr = allocate_memory(emu, 64).unwrap_or(0);
    if tm_addr != 0 {
        emu.maps.write_dword(tm_addr, 0); // tm_sec
        emu.maps.write_dword(tm_addr + 4, 30); // tm_min
        emu.maps.write_dword(tm_addr + 8, 12); // tm_hour
        emu.maps.write_dword(tm_addr + 12, 1); // tm_mday
        emu.maps.write_dword(tm_addr + 16, 0); // tm_mon
        emu.maps.write_dword(tm_addr + 20, 126); // tm_year (2026)
        emu.maps.write_dword(tm_addr + 24, 4); // tm_wday
        emu.maps.write_dword(tm_addr + 28, 0); // tm_yday
        emu.maps.write_dword(tm_addr + 32, 0); // tm_isdst
    }
    log_red!(
        emu,
        "msvcrt!localtime time_ptr:0x{:x} => 0x{:x}",
        time_ptr,
        tm_addr
    );
    emu.set_last_error(0);
    call.set_return(emu, tm_addr);
}
