use crate::emu;

pub fn NormalizeString(emu: &mut emu::Emu) {
    let norm_form = emu.regs().rcx;
    let src_ptr = emu.regs().rdx;
    let _src_len = emu.regs().r8 as i64;
    let dest_ptr = emu.regs().r9;
    let dest_len = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0) as i64;

    let src = emu.maps.read_wide_string(src_ptr);

    log_red!(
        emu,
        "kernel32!NormalizeString form:{} src:'{}' destLen:{}",
        norm_form,
        src,
        dest_len
    );

    let result_len = src.len() as i64;

    if dest_len == 0 || dest_ptr == 0 {
        emu.regs_mut().rax = (result_len + 1) as u64;
        return;
    }

    if dest_ptr != 0 && dest_len > 0 {
        emu.maps.write_wide_string(dest_ptr, &src);
    }
    emu.regs_mut().rax = (result_len + 1) as u64;
}
