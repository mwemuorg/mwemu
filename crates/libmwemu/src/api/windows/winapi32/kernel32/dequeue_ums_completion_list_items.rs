use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn DequeueUmsCompletionListItems(emu: &mut emu::Emu) {
    let ums_completion_list = arg(emu, 0) as u64;
    let wait_time_out = arg(emu, 1) as u64;
    let pp_ums_thread_list = arg(emu, 2) as u64;

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
    stdcall_ret(emu, 3);
}
