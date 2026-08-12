use crate::emu;

pub fn GetCommandLineA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetCommandlineA");

    let addr = emu
        .heap_mut()
        .allocate(2048)
        .expect("failed to allocate memory for GetCommandLineA");

    emu.maps.write_string(addr, "test.exe");
    emu.regs_mut().rax = addr;
}
