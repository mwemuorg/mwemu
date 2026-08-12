use crate::emu;

pub fn GetCommandLineW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetCommandLineW");

    let addr = emu
        .heap_mut()
        .allocate(2048)
        .expect("failed to allocate memory for GetCommandLineW");

    let exe_name = emu.cfg.exe_name.clone();
    emu.maps.write_wide_string(addr, &exe_name);
    emu.regs_mut().rax = addr;
}
