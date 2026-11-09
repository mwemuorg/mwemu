use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetCharWidth32W(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let first = arg(emu, 1);
    let last = arg(emu, 2);
    let buf = arg(emu, 3) as u64;
    let count = (last.saturating_sub(first) + 1).min(256);
    if buf != 0 {
        for i in 0..count {
            emu.maps.write_dword(buf + (i as u64) * 4, 8);
        }
    }
    log_red!(emu, "gdi32!GetCharWidth32W first:{} last:{}", first, last);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
