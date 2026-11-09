use crate::emu;

pub fn K32GetModuleInformation(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    let h_module = emu.regs().rdx;
    let lp_modinfo = emu.regs().r8;
    let cb = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!K32GetModuleInformation hProc:0x{:x} hMod:0x{:x} pInfo:0x{:x} cb:{}",
        h_process,
        h_module,
        lp_modinfo,
        cb
    );

    if lp_modinfo != 0 && cb >= 24 {
        emu.maps.write_qword(lp_modinfo, h_module);
        emu.maps.write_dword(lp_modinfo + 8, 0x1000);
        emu.maps.write_qword(lp_modinfo + 16, h_module);
    }
    emu.regs_mut().rax = 1;
}
