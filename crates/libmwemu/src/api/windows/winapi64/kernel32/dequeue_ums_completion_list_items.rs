use crate::emu;

pub fn DequeueUmsCompletionListItems(emu: &mut emu::Emu) {
    let ums_completion_list = emu.regs().rcx;
    let wait_time_out = emu.regs().rdx;
    let pp_ums_thread_list = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!DequeueUmsCompletionListItems list:0x{:x} timeout:{} ppThread:0x{:x}",
        ums_completion_list,
        wait_time_out,
        pp_ums_thread_list
    );

    if pp_ums_thread_list != 0 {
        emu.maps.write_qword(pp_ums_thread_list, 0);
    }
    emu.regs_mut().rax = 0;
}
