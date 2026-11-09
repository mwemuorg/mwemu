use crate::emu;

pub fn K32GetPerformanceInfo(emu: &mut emu::Emu) {
    let p_perf_info = emu.regs().rcx;
    let cb = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!K32GetPerformanceInfo pInfo:0x{:x} cb:{}",
        p_perf_info,
        cb
    );

    if p_perf_info != 0 && cb >= 104 {
        emu.maps.write_dword(p_perf_info, 104);
        emu.maps.write_qword(p_perf_info + 8, 0x100000);
        emu.maps.write_qword(p_perf_info + 16, 0x80000);
        emu.maps.write_qword(p_perf_info + 24, 0x200000);
        emu.maps.write_qword(p_perf_info + 32, 0x100000);
        emu.maps.write_dword(p_perf_info + 40, 100);
        emu.maps.write_dword(p_perf_info + 44, 1000);
        emu.maps.write_dword(p_perf_info + 48, 200);
        emu.maps.write_dword(p_perf_info + 52, 4096);
    }
    emu.regs_mut().rax = 1;
}
