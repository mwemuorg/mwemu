use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn GetSystemInfo(emu: &mut emu::Emu) {
    let out_sysinfo = WinApiAbi::arg(emu, 0);

    log_red!(emu, "kernel32!GetSystemInfo sysinfo: 0x{:x}", out_sysinfo);

    // let mut sysinfo = emu::structures::SystemInfo32::new();
    // sysinfo.save(out_sysinfo, &mut emu.maps);

    emu.stack_pop32(false);
}
