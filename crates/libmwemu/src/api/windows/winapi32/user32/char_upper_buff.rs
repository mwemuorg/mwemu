use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn CharUpperBuffA(emu: &mut emu::Emu) {
    char_upper_buff(emu, "CharUpperBuffA", false);
}

pub(super) fn CharUpperBuffW(emu: &mut emu::Emu) {
    char_upper_buff(emu, "CharUpperBuffW", true);
}

fn char_upper_buff(emu: &mut emu::Emu, api: &str, wide: bool) {
    let buf = arg(emu, 0) as u64;
    let len = arg(emu, 1) as u64;
    log_red!(emu, "user32!{api} buf=0x{:x} len={}", buf, len);
    if wide {
        for i in 0..len {
            if let Some(ch) = emu.maps.read_word(buf + i * 2)
                && let Some(c) = char::from_u32(ch as u32)
            {
                let out = c.to_uppercase().next().unwrap_or(c) as u32;
                emu.maps.write_word(buf + i * 2, out as u16);
            }
        }
    } else {
        for i in 0..len {
            if let Some(ch) = emu.maps.read_byte(buf + i) {
                emu.maps.write_byte(buf + i, ch.to_ascii_uppercase());
            }
        }
    }

    emu.regs_mut().rax = len;
    stdcall_ret(emu, 2);
}
