use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::structures;

pub fn GetNativeSystemInfo(emu: &mut emu::Emu) {
    let sysinfo_ptr = WinApiAbi::arg(emu, 0);

    let mut sysinfo = structures::SystemInfo32::new();
    sysinfo.save(sysinfo_ptr, &mut emu.maps);

    log_red!(emu, "kernel32!GetNativeSystemInfo");

    emu.stack_pop32(false);
}
