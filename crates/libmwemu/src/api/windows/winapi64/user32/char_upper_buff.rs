use crate::emu;

pub(super) fn CharUpperBuffA(emu: &mut emu::Emu) {
    char_upper_buff(emu, "CharUpperBuffA", false);
}

pub(super) fn CharUpperBuffW(emu: &mut emu::Emu) {
    char_upper_buff(emu, "CharUpperBuffW", true);
}

fn char_upper_buff(emu: &mut emu::Emu, api: &str, wide: bool) {
    let buf = emu.regs().rcx;
    let len = emu.regs().rdx as u32;
    log_red!(emu, "user32!{api} buf=0x{:x} len={}", buf, len);
    if wide {
        for i in 0..len as u64 {
            if let Some(ch) = emu.maps.read_word(buf + i * 2)
                && let Some(c) = char::from_u32(ch as u32)
            {
                let upper = c.to_uppercase().next().unwrap_or(c) as u16;
                emu.maps.write_word(buf + i * 2, upper);
            }
        }
    } else {
        for i in 0..len as u64 {
            if let Some(ch) = emu.maps.read_byte(buf + i) {
                emu.maps
                    .write_byte(buf + i, (ch as char).to_ascii_uppercase() as u8);
            }
        }
    }

    emu.regs_mut().rax = len as u64;
}
