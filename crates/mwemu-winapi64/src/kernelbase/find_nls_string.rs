use crate::emu;

pub fn FindNLSString(emu: &mut emu::Emu) {
    let locale_id = emu.regs().rcx as u32;
    let flags = emu.regs().rdx as u32;
    let src_ptr = emu.regs().r8;
    let src_count = emu.regs().r9 as i32;
    let src = emu.maps.read_wide_string(src_ptr);
    log_red!(
        emu,
        "kernelbase!FindNLSString locale:{} flags:0x{:x} src:'{}'",
        locale_id,
        flags,
        src
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = -1i64 as u64; // not found
}
