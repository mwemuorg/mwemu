use crate::emu;

pub fn LCMapStringA(emu: &mut emu::Emu) {
    let _locale = emu.regs().rcx as u32;
    let flags = emu.regs().rdx as u32;
    let src_ptr = emu.regs().r8;
    let _src_len = emu.regs().r9 as i32;
    let dst_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let dst_len = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as i32;
    let src = if src_ptr != 0 {
        emu.maps.read_string(src_ptr)
    } else {
        String::new()
    };
    let mapped = if flags & 0x00000100 != 0 {
        src.to_uppercase()
    } else if flags & 0x00000200 != 0 {
        src.to_lowercase()
    } else {
        src.clone()
    };
    let result_len = mapped.len() + 1;
    if dst_ptr != 0 && dst_len > 0 {
        emu.maps.write_string(dst_ptr, &mapped);
    }
    log_red!(
        emu,
        "kernelbase!LCMapStringA flags:0x{:x} '{}' => '{}'",
        flags,
        src,
        mapped
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = result_len as u64;
}
