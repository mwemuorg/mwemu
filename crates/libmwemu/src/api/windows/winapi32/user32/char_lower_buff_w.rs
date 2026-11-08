use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn CharLowerBuffW(emu: &mut emu::Emu) {
    let buf = arg(emu, 0) as u64;
    let len = arg(emu, 1) as u64;
    log_red!(emu, "user32!CharLowerBuffW buf=0x{:x} len={}", buf, len);
    for i in 0..len {
        if let Some(ch) = emu.maps.read_word(buf + i * 2)
            && let Some(c) = char::from_u32(ch as u32)
        {
            let out = c.to_lowercase().next().unwrap_or(c) as u32;
            emu.maps.write_word(buf + i * 2, out as u16);
        }
    }
    emu.regs_mut().rax = len;
    stdcall_ret(emu, 2);
}
