use crate::emu;

pub fn GetProcessPreferredUILanguages(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx as u32;
    let num_languages_ptr = emu.regs().rdx;
    let languages_buf = emu.regs().r8;
    let buf_size_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetProcessPreferredUILanguages flags:0x{:x} num:0x{:x} buf:0x{:x} size:0x{:x}",
        flags,
        num_languages_ptr,
        languages_buf,
        buf_size_ptr
    );
    if num_languages_ptr != 0 {
        emu.maps.write_dword(num_languages_ptr, 1);
    }
    if languages_buf != 0 {
        emu.maps.write_wide_string(languages_buf, "en-US");
        emu.maps.write_word(languages_buf + 12, 0);
    }
    if buf_size_ptr != 0 {
        emu.maps.write_dword(buf_size_ptr, 7);
    }
    emu.regs_mut().rax = 1;
}
