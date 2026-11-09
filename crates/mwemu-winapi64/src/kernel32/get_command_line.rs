use crate::emu;

pub fn GetCommandLineA(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "kernel32!GetCommandLineA");

    let addr = match emu.heap_mut().allocate(2048) {
        Some(a) => a,
        None => {
            log::error!("kernel32!GetCommandLineA: out of memory");
            emu.regs_mut().rax = 0;
            return;
        }
    };

    let exe_name = emu.cfg.exe_name.clone();
    emu.maps.write_string(addr, &exe_name);
    emu.regs_mut().rax = addr;
}

pub fn GetCommandLineW(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "kernel32!GetCommandLineW");

    let addr = match emu.heap_mut().allocate(2048) {
        Some(a) => a,
        None => {
            log::error!("kernel32!GetCommandLineW: out of memory");
            emu.regs_mut().rax = 0;
            return;
        }
    };

    let exe_name = emu.cfg.exe_name.clone();
    emu.maps.write_wide_string(addr, &exe_name);
    emu.regs_mut().rax = addr;
}
