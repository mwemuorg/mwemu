
use crate::emu;
use crate::winapi::helper;
pub fn CompareFileTime(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let ft1_low = emu.maps.read_dword(p0).unwrap_or(0) as u64;
    let ft1_high = emu.maps.read_dword(p0 + 4).unwrap_or(0) as u64;
    let ft2_low = emu.maps.read_dword(p1).unwrap_or(0) as u64;
    let ft2_high = emu.maps.read_dword(p1 + 4).unwrap_or(0) as u64;
    let t1 = (ft1_high << 32) | ft1_low;
    let t2 = (ft2_high << 32) | ft2_low;
    let result: i64 = if t1 < t2 {
        -1
    } else if t1 > t2 {
        1
    } else {
        0
    };
    log_red!(
        emu,
        "kernel32!CompareFileTime 0x{:x} vs 0x{:x} => {}",
        t1,
        t2,
        result
    );
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = result as u64;
}
