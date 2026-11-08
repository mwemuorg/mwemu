use crate::emu;
use crate::windows::structures;

pub fn GetNativeSystemInfo(emu: &mut emu::Emu) {
    let sysinfo_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    let mut sysinfo = structures::SystemInfo32::new();
    sysinfo.save(sysinfo_ptr, &mut emu.maps);

    log_red!(emu, "kernel32!GetNativeSystemInfo");

    emu.stack_pop32(false);
}
