use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn ResolveLocaleName(emu: &mut emu::Emu) {
    let p0 = arg(emu, 0) as u64;
    let p1 = arg(emu, 1) as u64;
    let p2 = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!ResolveLocaleName p1:0x{:x} p2:0x{:x}",
        p1,
        p2
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
