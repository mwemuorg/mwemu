use crate::emu;

pub fn UnionRect(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src1 = emu.regs().rdx;
    let src2 = emu.regs().r8;
    if dst != 0 && src1 != 0 && src2 != 0 {
        let l1 = emu.maps.read_dword(src1).unwrap_or(0) as i32;
        let t1 = emu.maps.read_dword(src1 + 4).unwrap_or(0) as i32;
        let r1 = emu.maps.read_dword(src1 + 8).unwrap_or(0) as i32;
        let b1 = emu.maps.read_dword(src1 + 12).unwrap_or(0) as i32;
        let l2 = emu.maps.read_dword(src2).unwrap_or(0) as i32;
        let t2 = emu.maps.read_dword(src2 + 4).unwrap_or(0) as i32;
        let r2 = emu.maps.read_dword(src2 + 8).unwrap_or(0) as i32;
        let b2 = emu.maps.read_dword(src2 + 12).unwrap_or(0) as i32;
        emu.maps.write_dword(dst, std::cmp::min(l1, l2) as u32);
        emu.maps.write_dword(dst + 4, std::cmp::min(t1, t2) as u32);
        emu.maps.write_dword(dst + 8, std::cmp::max(r1, r2) as u32);
        emu.maps.write_dword(dst + 12, std::cmp::max(b1, b2) as u32);
    }
    log_red!(
        emu,
        "user32!UnionRect dst:0x{:x} s1:0x{:x} s2:0x{:x}",
        dst,
        src1,
        src2
    );
    emu.regs_mut().rax = 1;
}
