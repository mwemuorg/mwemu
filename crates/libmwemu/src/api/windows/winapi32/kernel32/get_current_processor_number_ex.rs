use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetCurrentProcessorNumberEx(emu: &mut emu::Emu) {
    let proc_num_ptr = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!GetCurrentProcessorNumberEx ptr:0x{:x}",
        proc_num_ptr
    );
    if proc_num_ptr != 0 {
        emu.maps.write_word(proc_num_ptr, 0);
        emu.maps.write_byte(proc_num_ptr + 2, 0);
        emu.maps.write_byte(proc_num_ptr + 3, 0);
    }
    stdcall_ret(emu, 1);
}
