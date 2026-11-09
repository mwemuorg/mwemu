use crate::emu;

pub fn GetLocaleInfoEx(emu: &mut emu::Emu) {
    let locale_ptr = emu.regs().rcx;
    let lctype = emu.regs().rdx;
    let buf_ptr = emu.regs().r8;
    let buf_size = emu.regs().r9;
    let locale = emu.maps.read_wide_string(locale_ptr);
    if buf_ptr != 0 && buf_size > 0 {
        emu.maps.write_wide_string(buf_ptr, "en-US");
    }
    log_red!(
        emu,
        "kernelbase!GetLocaleInfoEx '{}' type:0x{:x}",
        locale,
        lctype
    );
    emu.regs_mut().rax = 6;
}
