use crate::emu;

pub fn GetCommandLineA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetCommandLineA");

    let addr = emu
        .heap_mut()
        .allocate(2048)
        .expect("failed to allocate memory for GetCommandLineA");

    let exe_name = emu.cfg.exe_name.clone();
    emu.maps.write_string(addr, &exe_name);
    emu.regs_mut().rax = addr;
}
