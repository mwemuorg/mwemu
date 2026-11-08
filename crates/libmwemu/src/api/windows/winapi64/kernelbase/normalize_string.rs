use crate::emu;

pub fn NormalizeString(emu: &mut emu::Emu) {
    let norm_form = emu.regs().rcx as u32;
    let src_ptr = emu.regs().rdx;
    let src_len = emu.regs().r8 as i32;
    let dst_ptr = emu.regs().r9;
    let dst_len = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
    let src = emu.maps.read_wide_string(src_ptr);
    let actual_len = (src.len() + 1) as i32;
    if dst_len == 0 {
        log_red!(
            emu,
            "kernelbase!NormalizeString form:{} '{}' => need {} chars",
            norm_form,
            src,
            actual_len
        );
        emu.regs_mut().rax = actual_len as u64;
    } else {
        if dst_ptr != 0 {
            emu.maps.write_wide_string(dst_ptr, &src);
        }
        log_red!(
            emu,
            "kernelbase!NormalizeString form:{} '{}' => {}",
            norm_form,
            src,
            actual_len
        );
        emu.set_last_error(0);
        emu.regs_mut().rax = actual_len as u64;
    }
}
