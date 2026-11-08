use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetSystemDefaultLocaleName(emu: &mut emu::Emu) {
    let buf_ptr = arg(emu, 0) as u64;
    let buf_len = arg(emu, 1);
    log_red!(
        emu,
        "kernel32!GetSystemDefaultLocaleName ptr:0x{:x} len:{}",
        buf_ptr,
        buf_len
    );
    let out_ptr = arg(emu, 1) as u64;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    if buf_ptr != 0 && buf_len >= 6 {
        emu.maps.write_wide_string(buf_ptr, "en-US");
    }
    emu.regs_mut().rax = 6;
    stdcall_ret(emu, 2);
}
