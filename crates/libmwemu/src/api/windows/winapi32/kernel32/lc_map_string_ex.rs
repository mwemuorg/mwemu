use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn LCMapStringEx(emu: &mut emu::Emu) {
    let p0 = arg(emu, 0) as u64;
    let p1 = arg(emu, 1) as u64;
    let p2 = arg(emu, 2) as u64;
    let p3 = arg(emu, 3) as u64;
    let p4 = arg(emu, 4) as u64;
    let p5 = arg(emu, 5) as u64;
    let p6 = arg(emu, 6) as u64;
    let p7 = arg(emu, 7) as u64;
    log_red!(
        emu,
        "kernel32!LCMapStringEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        p1,
        p2,
        p3
    );
    let handle = helper::handler_create("LcMapStringEx");
    let out_ptr = arg(emu, 7) as u64;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 8);
}
