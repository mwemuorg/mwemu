use crate::emu;

pub fn GetSystemInfo(emu: &mut emu::Emu) {
    let out_sysinfo = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!GetSystemInfo cannot read out_sysinfo") as u64;

    log_red!(emu, "kernel32!GetSystemInfo sysinfo: 0x{:x}", out_sysinfo);

    // let mut sysinfo = emu::structures::SystemInfo32::new();
    // sysinfo.save(out_sysinfo, &mut emu.maps);

    emu.stack_pop32(false);
}
