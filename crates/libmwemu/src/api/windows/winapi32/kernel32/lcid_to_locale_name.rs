use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn LCIDToLocaleName(emu: &mut emu::Emu) {
    let p0 = arg(emu, 0) as u64;
    let p1 = arg(emu, 1) as u64;
    let p2 = arg(emu, 2) as u64;
    let p3 = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!LCIDToLocaleName p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        p1,
        p2,
        p3
    );
    let first_param = arg(emu, 0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
