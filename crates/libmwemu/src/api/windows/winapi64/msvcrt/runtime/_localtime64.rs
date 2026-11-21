use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _localtime64(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let time_ptr = api_try!(emu, call.arg(emu, 0));
    let _time_val = if time_ptr != 0 {
        emu.maps.read_qword(time_ptr).unwrap_or(0)
    } else {
        0
    };
    let tm_addr = emu.maps.alloc(36).unwrap_or_default();
    if tm_addr != 0
        && emu
            .maps
            .create_map(
                "tm_struct",
                tm_addr,
                36,
                crate::maps::mem64::Permission::from_flags(true, true, false),
            )
            .is_ok()
    {
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
    log_red!(emu, "msvcrt!_localtime64 => 0x{:x}", tm_addr);
    emu.set_last_error(0);
    call.set_return(emu, tm_addr);
}
