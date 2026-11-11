use crate::emu;

pub(super) fn CharLowerBuffA(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let len = emu.regs().rdx as u32;
    log_red!(emu, "user32!CharLowerBuffA buf=0x{:x} len={}", buf, len);
    for i in 0..len as u64 {
        if let Some(ch) = emu.maps.read_byte(buf + i) {
            emu.maps
                .write_byte(buf + i, (ch as char).to_ascii_lowercase() as u8);
        }
    }
    emu.regs_mut().rax = len as u64;
}

pub(super) fn CharLowerBuffW(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let len = emu.regs().rdx as u32;
    log_red!(emu, "user32!CharLowerBuffW buf=0x{:x} len={}", buf, len);
    for i in 0..len as u64 {
        if let Some(ch) = emu.maps.read_word(buf + i * 2)
            && let Some(c) = char::from_u32(ch as u32)
        {
            let lower = c.to_lowercase().next().unwrap_or(c) as u16;
            emu.maps.write_word(buf + i * 2, lower);
        }
    }
    emu.regs_mut().rax = len as u64;
}
