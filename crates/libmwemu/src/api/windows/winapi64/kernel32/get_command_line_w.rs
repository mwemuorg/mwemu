use crate::emu;

pub fn GetCommandLineW(emu: &mut emu::Emu) {
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
