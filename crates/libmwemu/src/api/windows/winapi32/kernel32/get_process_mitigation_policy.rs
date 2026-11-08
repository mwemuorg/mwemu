use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetProcessMitigationPolicy(emu: &mut emu::Emu) {
    let h_process = arg(emu, 0) as u64;
    let policy = arg(emu, 1);
    let buf_ptr = arg(emu, 2) as u64;
    let buf_len = arg(emu, 3);
    log_red!(
        emu,
        "kernel32!GetProcessMitigationPolicy h:0x{:x} policy:{} ptr:0x{:x} len:{}",
        h_process,
        policy,
        buf_ptr,
        buf_len
    );
    let out_ptr = arg(emu, 3) as u64;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    if buf_ptr != 0 && buf_len > 0 {
        let clear_size = (buf_len as usize).min(64);
        for i in 0..clear_size {
            emu.maps.write_byte(buf_ptr + i as u64, 0);
        }
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
