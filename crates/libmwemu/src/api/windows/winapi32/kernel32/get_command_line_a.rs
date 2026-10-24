use crate::emu;

pub fn GetCommandLineA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetCommandlineA");

    let addr = match emu.heap_mut().allocate(2048) {
        Some(a) => a,
        None => {
            log::error!("kernel32!GetCommandLineA: out of memory");
            emu.regs_mut().rax = 0;
            return;
        }
    };

    emu.maps.write_string(addr, "test.exe");
    emu.regs_mut().rax = addr;
}
