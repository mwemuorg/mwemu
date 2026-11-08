use crate::emu;

pub fn GetSystemDefaultLocaleName(emu: &mut emu::Emu) {
    let buf_ptr = emu.regs().rcx;
    let cch_buf = emu.regs().rdx as usize;
    let locale = "en-US";
    if buf_ptr != 0 {
        emu.maps.write_wide_string(buf_ptr, locale);
    }
    log_red!(emu, "kernelbase!GetSystemDefaultLocaleName => '{}'", locale);
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = (locale.len() + 1) as u64;
}
