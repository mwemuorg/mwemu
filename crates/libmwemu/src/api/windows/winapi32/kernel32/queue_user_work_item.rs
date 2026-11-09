use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn QueueUserWorkItem(emu: &mut emu::Emu) {
    let function = arg(emu, 0) as u64;
    let context = arg(emu, 1) as u64;
    let flags = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!QueueUserWorkItem func:0x{:x} ctx:0x{:x} flags:0x{:x}",
        function,
        context,
        flags
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
