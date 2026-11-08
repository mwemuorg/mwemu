use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetThreadIdealProcessorEx(emu: &mut emu::Emu) {
    let h_thread = arg(emu, 0) as u64;
    let proc_num_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!GetThreadIdealProcessorEx h:0x{:x} ptr:0x{:x}",
        h_thread,
        proc_num_ptr
    );
    if proc_num_ptr != 0 {
        emu.maps.write_word(proc_num_ptr, 0);
        emu.maps.write_byte(proc_num_ptr + 2, 0);
        emu.maps.write_byte(proc_num_ptr + 3, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
