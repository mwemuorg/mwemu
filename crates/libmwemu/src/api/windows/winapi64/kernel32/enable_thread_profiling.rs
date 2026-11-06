
use crate::emu;
use crate::winapi::helper;
pub fn EnableThreadProfiling(emu: &mut emu::Emu) {
    let h_thread = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let hardware_counters = emu.regs().r8;
    let p_perf_data_handle = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!EnableThreadProfiling hThread:0x{:x} flags:0x{:x} hw:0x{:x} pHandle:0x{:x}",
        h_thread,
        flags,
        hardware_counters,
        p_perf_data_handle
    );
    let handle = helper::handler_create("EnableThreadProfiling");
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
