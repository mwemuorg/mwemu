use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetUmsSystemThreadInformation(emu: &mut emu::Emu) {
    let h_thread = arg(emu, 0) as u64;
    let info_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!GetUmsSystemThreadInformation h:0x{:x} ptr:0x{:x}",
        h_thread,
        info_ptr
    );
    let out_ptr = arg(emu, 1) as u64;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}
