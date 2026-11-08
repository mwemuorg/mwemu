use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn _hwrite(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    let buffer = arg(emu, 1) as u64;
    let count = arg(emu, 2) as u64 as usize;
    log_red!(
        emu,
        "kernel32!_hwrite handle:0x{:x} buffer:0x{:x} count:{}",
        handle,
        buffer,
        count
    );
    if helper::handler_exist(handle) && buffer != 0 && count > 0 {
        emu.set_last_error(0);
        emu.regs_mut().rax = count as u64;
    } else {
        emu.regs_mut().rax = -1i64 as u64;
    }
    stdcall_ret(emu, 3);
}
