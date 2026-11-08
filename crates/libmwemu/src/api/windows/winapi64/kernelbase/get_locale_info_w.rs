use crate::emu;

pub fn GetLocaleInfoW(emu: &mut emu::Emu) {
    let locale = emu.regs().rcx;
    let lctype = emu.regs().rdx;
    let buf_ptr = emu.regs().r8;
    let buf_size = emu.regs().r9;
    if buf_ptr != 0 && buf_size > 0 {
        emu.maps.write_wide_string(buf_ptr, "en-US");
    }
    log_red!(
        emu,
        "kernelbase!GetLocaleInfoW locale:0x{:x} type:0x{:x}",
        locale,
        lctype
    );
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.regs_mut().rax = 6;
}
