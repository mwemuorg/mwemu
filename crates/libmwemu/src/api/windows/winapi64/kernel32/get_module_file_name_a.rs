use crate::emu;

pub fn GetModuleFileNameA(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let out_filename = emu.regs().rdx;
    let sz = emu.regs().r8;

    if sz >= 11 {
        let exe_name = emu.cfg.exe_name.clone();
        emu.maps.write_string(out_filename, &exe_name);
        emu.regs_mut().rax = 11;
    } else {
        emu.regs_mut().rax = 0;
    }

    log_red!(emu, "kernel32!GetModuleFileNameA hndl:{:x}", hndl);
}
