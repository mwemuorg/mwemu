use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetProcessHeaps(emu: &mut emu::Emu) {
    let num_heaps = arg(emu, 0);
    let heaps_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!GetProcessHeaps count:{} ptr:0x{:x}",
        num_heaps,
        heaps_ptr
    );
    if heaps_ptr != 0 && num_heaps >= 1 {
        emu.maps.write_qword(heaps_ptr, 0x00050000);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
