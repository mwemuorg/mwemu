use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetProcessGroupAffinity(emu: &mut emu::Emu) {
    let h_process = arg(emu, 0) as u64;
    let count_ptr = arg(emu, 1) as u64;
    let array_ptr = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!GetProcessGroupAffinity h:0x{:x} count:0x{:x} array:0x{:x}",
        h_process,
        count_ptr,
        array_ptr
    );
    if count_ptr != 0 {
        emu.maps.write_word(count_ptr, 1);
    }
    if array_ptr != 0 {
        emu.maps.write_word(array_ptr, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
