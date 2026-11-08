use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn RegEnumKeyExW(emu: &mut emu::Emu) {
    let p0 = arg(emu, 0) as u64;
    let p1 = arg(emu, 1) as u64;
    let p2 = arg(emu, 2) as u64;
    let p3 = arg(emu, 3) as u64;
    let p4 = arg(emu, 4) as u64;
    let p5 = arg(emu, 5) as u64;
    let p6 = arg(emu, 6) as u64;
    let p7 = arg(emu, 7) as u64;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernel32!RegEnumKeyExW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 259;
    stdcall_ret(emu, 8);
}
