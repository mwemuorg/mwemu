use crate::emu;

pub fn GetCommandLineW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetCommandlineW");

    let addr = match emu.heap_mut().allocate(2048) {
        Some(a) => a,
        None => {
            log::error!("kernel32!GetCommandLineW: out of memory");
            emu.regs_mut().rax = 0;
            return;
        }
    };

    emu.maps.write_wide_string(addr, "test.exe");
    emu.regs_mut().rax = addr;
}
