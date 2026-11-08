use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetProcessInformation(emu: &mut emu::Emu) {
    let h_process = arg(emu, 0) as u64;
    let info_class = arg(emu, 1);
    let info_ptr = arg(emu, 2) as u64;
    let info_size = arg(emu, 3);
    log_red!(
        emu,
        "kernel32!GetProcessInformation h:0x{:x} class:{} ptr:0x{:x} size:{}",
        h_process,
        info_class,
        info_ptr,
        info_size
    );
    let out_ptr = arg(emu, 3) as u64;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    if info_ptr != 0 && info_size > 0 {
        let clear_size = (info_size as usize).min(256);
        for i in 0..clear_size {
            emu.maps.write_byte(info_ptr + i as u64, 0);
        }
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
