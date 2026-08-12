use crate::emu;

pub fn GetCommandLineW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetCommandlineW");

    let addr = emu
        .heap_mut()
        .allocate(2048)
        .expect("failed to allocate memory for GetCommandLineW");

    emu.maps.write_wide_string(addr, "test.exe");
    emu.regs_mut().rax = addr;
}
