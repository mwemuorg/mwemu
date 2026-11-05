use crate::emu;

pub fn IsProcessInJob(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    let h_job = emu.regs().rdx;
    let p_result = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!IsProcessInJob hProcess:0x{:x} hJob:0x{:x} pResult:0x{:x}",
        h_process,
        h_job,
        p_result
    );
    if p_result != 0 {
        emu.maps.write_dword(p_result, 0);
    }
    emu.regs_mut().rax = 1;
}
