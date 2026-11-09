use crate::emu;

pub fn GetLocaleInfoEx(emu: &mut emu::Emu) {
    let locale_name = emu.regs().rcx;
    let lc_type = emu.regs().rdx;
    let buf = emu.regs().r8;
    let buf_len = emu.regs().r9;
    let name_s = if locale_name != 0 {
        emu.maps.read_wide_string(locale_name)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernel32!GetLocaleInfoEx '{}' type:0x{:x} buf_len:{}",
        name_s,
        lc_type,
        buf_len
    );
    if buf != 0 && buf_len > 0 {
        emu.maps.write_wide_string(buf, "en-US");
        emu.regs_mut().rax = 6; // length of "en-US" + null
    } else {
        emu.regs_mut().rax = 6;
    }
}
