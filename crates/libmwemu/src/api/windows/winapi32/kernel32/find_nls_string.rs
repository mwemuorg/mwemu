use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn FindNLSString(emu: &mut emu::Emu) {
    let locale = arg(emu, 0) as u64 as u32;
    let flags = arg(emu, 1) as u64 as u32;
    let source_ptr = arg(emu, 2) as u64;
    let source_count = arg(emu, 3) as u64 as i32;
    log_red!(
        emu,
        "kernel32!FindNLSString locale:0x{:x} flags:0x{:x} src:0x{:x}",
        locale,
        flags,
        source_ptr
    );
    let out_ptr = arg(emu, 3) as u64;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = (-1i64) as u64;
    stdcall_ret(emu, 4);
}
