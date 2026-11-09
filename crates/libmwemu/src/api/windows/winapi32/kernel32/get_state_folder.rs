use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetStateFolder(emu: &mut emu::Emu) {
    let state_type = arg(emu, 0);
    let buf_ptr = arg(emu, 1) as u64;
    let buf_len_ptr = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!GetStateFolder type:{} buf:0x{:x} len:0x{:x}",
        state_type,
        buf_ptr,
        buf_len_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x80070057u64;
    stdcall_ret(emu, 3);
}
