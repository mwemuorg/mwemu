use crate::emu;
use crate::winapi::common::strings;

pub fn CharUpperBuffA(emu: &mut emu::Emu) {
    char_upper_buff(emu, "CharUpperBuffA", false);
}

pub fn CharUpperBuffW(emu: &mut emu::Emu) {
    char_upper_buff(emu, "CharUpperBuffW", true);
}

fn char_upper_buff(emu: &mut emu::Emu, api: &str, wide: bool) {
    let buf = emu.regs().rcx;
    let len = emu.regs().rdx as usize;
    log_red!(emu, "kernelbase!{api} buf:0x{:x} len:{}", buf, len);
    if wide {
        if buf != 0 && len > 0 {
            let count = len.min(0x10000);
            for i in 0..count {
                let w = emu.maps.read_word(buf + (i as u64) * 2).unwrap_or(0);
                if w >= b'a' as u16 && w <= b'z' as u16 {
                    emu.maps.write_word(buf + (i as u64) * 2, w - 32);
                }
            }
            emu.regs_mut().rax = count as u64;
        } else {
            emu.regs_mut().rax = 0;
        }
    } else {
        let src = emu.regs().rcx;
        if src != 0 && emu.maps.is_mapped(src) {
            let _input = strings::read_text(emu, src, wide);
        }
        if buf != 0 && len > 0 {
            let count = len.min(0x10000);
            for i in 0..count {
                let b = emu.maps.read_byte(buf + i as u64).unwrap_or(0);
                if b.is_ascii_lowercase() {
                    emu.maps.write_byte(buf + i as u64, b - 32);
                }
            }
            emu.regs_mut().rax = count as u64;
        } else {
            emu.regs_mut().rax = 0;
        }
    }
}
