use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn FindStringOrdinal(emu: &mut emu::Emu) {
    let flags = arg(emu, 0) as u64 as u32;
    let source_ptr = arg(emu, 1) as u64;
    let source_count = arg(emu, 2) as u64 as i32;
    let value_ptr = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!FindStringOrdinal flags:0x{:x} src:0x{:x} val:0x{:x}",
        flags,
        source_ptr,
        value_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = (-1i64) as u64;
    stdcall_ret(emu, 4);
}
