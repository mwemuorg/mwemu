use crate::emu;

pub fn CharToOemBuffA(emu: &mut emu::Emu) {
    let src = emu.regs().rcx;
    let dst = emu.regs().rdx;
    let len = emu.regs().r8 as u32;
    log_red!(
        emu,
        "user32!CharToOemBuffA src: 0x{:x} dst: 0x{:x} len: {}",
        src,
        dst,
        len
    );
    let src = emu.regs().rcx;
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = emu.maps.read_string(src);
    }
    if src != 0 && dst != 0 {
        for i in 0..len as u64 {
            let b = emu.maps.read_byte(src + i).unwrap_or(0);
            emu.maps.write_byte(dst + i, b);
        }
    }
    emu.regs_mut().rax = 1; // TRUE
}

pub fn CharToOemBuffW(emu: &mut emu::Emu) {
    let src = emu.regs().rcx;
    let dst = emu.regs().rdx;
    let len = emu.regs().r8 as u32;
    log_red!(
        emu,
        "user32!CharToOemBuffW src: 0x{:x} dst: 0x{:x} len: {}",
        src,
        dst,
        len
    );
    let src = emu.regs().rcx;
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = emu.maps.read_string(src);
    }
    if src != 0 && dst != 0 {
        for i in 0..len as u64 {
            let ch = emu.maps.read_word(src + i * 2).unwrap_or(0);
            emu.maps.write_byte(dst + i, (ch & 0xFF) as u8);
        }
    }
    emu.regs_mut().rax = 1; // TRUE
}
