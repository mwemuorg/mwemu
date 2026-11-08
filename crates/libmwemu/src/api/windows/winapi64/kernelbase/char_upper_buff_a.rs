use crate::emu;

pub fn CharUpperBuffA(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let len = emu.regs().rdx as usize;
    log_red!(emu, "kernelbase!CharUpperBuffA buf:0x{:x} len:{}", buf, len);
    let src = emu.regs().rcx;
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = emu.maps.read_string(src);
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
