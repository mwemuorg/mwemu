use crate::emu;

pub fn LCMapStringW(emu: &mut emu::Emu) {
    let locale = emu.maps.read_dword(emu.regs().rsp).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().rsp + 4).unwrap_or(0);
    let src_ptr = emu.maps.read_dword(emu.regs().rsp + 8).unwrap_or(0) as u64;
    let src_sz = emu.maps.read_dword(emu.regs().rsp + 12).unwrap_or(0);
    let dest_ptr = emu.maps.read_dword(emu.regs().rsp + 16).unwrap_or(0) as u64;
    let dest_sz = emu.maps.read_dword(emu.regs().rsp + 20).unwrap_or(0);

    let s = emu.maps.read_wide_string(src_ptr);
    let char_count = if src_sz as i32 == -1 {
        s.len() as u32 + 1
    } else {
        src_sz
    };

    log_red!(
        emu,
        "kernel32!LCMapStringW `{}` dst:0x{:x} sz:{}->{}",
        s,
        dest_ptr,
        src_sz,
        dest_sz
    );

    if dest_ptr > 0 && dest_sz > 0 {
        emu.maps.write_wide_string(dest_ptr, &s);
    }

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = char_count as u64;
}
