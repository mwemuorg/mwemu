use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetThreadGroupAffinity(emu: &mut emu::Emu) {
    let h_thread = arg(emu, 0) as u64;
    let affinity_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!GetThreadGroupAffinity h:0x{:x} ptr:0x{:x}",
        h_thread,
        affinity_ptr
    );
    if affinity_ptr != 0 {
        emu.maps.write_qword(affinity_ptr, 0xF);
        emu.maps.write_word(affinity_ptr + 8, 0);
        for i in 0..3u64 {
            emu.maps.write_word(affinity_ptr + 10 + i * 2, 0);
        }
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
