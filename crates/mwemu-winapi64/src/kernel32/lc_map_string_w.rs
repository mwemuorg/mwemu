use crate::emu;

pub fn LCMapStringW(emu: &mut emu::Emu) {
    let locale = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let src = emu.regs().r8;
    let src_len = emu.regs().r9 as i64;
    let dst = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let dst_len = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as i64;
    let real_src_len = if src_len < 0 {
        let s = emu.maps.read_wide_string(src);
        (s.len() + 1) as i64
    } else {
        src_len
    };
    log_red!(
        emu,
        "kernel32!LCMapStringW locale: 0x{:x} flags: 0x{:x} src_len: {} dst_len: {}",
        locale,
        flags,
        real_src_len,
        dst_len
    );
    if dst == 0 || dst_len == 0 {
        emu.regs_mut().rax = real_src_len as u64;
    } else {
        let copy_len = std::cmp::min(real_src_len, dst_len) as u64;
        for i in 0..copy_len * 2 {
            let b = emu.maps.read_byte(src + i).unwrap_or(0);
            emu.maps.write_byte(dst + i, b);
        }
        emu.regs_mut().rax = copy_len;
    }
}
