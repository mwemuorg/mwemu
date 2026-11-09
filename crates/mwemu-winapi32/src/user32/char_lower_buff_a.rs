use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn CharLowerBuffA(emu: &mut emu::Emu) {
    let buf = arg(emu, 0) as u64;
    let len = arg(emu, 1) as u64;
    log_red!(emu, "user32!CharLowerBuffA buf=0x{:x} len={}", buf, len);
    for i in 0..len {
        if let Some(ch) = emu.maps.read_byte(buf + i) {
            emu.maps.write_byte(buf + i, ch.to_ascii_lowercase());
        }
    }
    emu.regs_mut().rax = len;
    stdcall_ret(emu, 2);
}
