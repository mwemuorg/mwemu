use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetSystemTimePreciseAsFileTime(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ft_ptr = api_try!(emu, call.arg(emu, 0));

    log_red!(
        emu,
        "kernel32!GetSystemTimePreciseAsFileTime ptr:0x{:x}",
        ft_ptr
    );

    // Base: 2024-01-01 00:00:00 UTC + tick-based offset
    let base_ft: u64 = 0x01DA_5E3E_A46C_0000;
    let filetime = base_ft + (emu.tick as u64) * 10_000;

    if ft_ptr != 0 {
        emu.maps
            .write_dword(ft_ptr, (filetime & 0xFFFF_FFFF) as u32);
        emu.maps.write_dword(ft_ptr + 4, (filetime >> 32) as u32);
    }
}
