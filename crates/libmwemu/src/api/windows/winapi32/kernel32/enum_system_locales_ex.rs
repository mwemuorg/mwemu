use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn EnumSystemLocalesEx(emu: &mut emu::Emu) {
    let p0 = arg(emu, 0) as u64;
    let p1 = arg(emu, 1) as u64;
    let p2 = arg(emu, 2) as u64;
    let p3 = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!EnumSystemLocalesEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        p1,
        p2,
        p3
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
