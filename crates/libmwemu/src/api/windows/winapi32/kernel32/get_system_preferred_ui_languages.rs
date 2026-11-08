use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetSystemPreferredUILanguages(emu: &mut emu::Emu) {
    let flags = arg(emu, 0);
    let num_languages_ptr = arg(emu, 1) as u64;
    let languages_buf = arg(emu, 2) as u64;
    let buf_size_ptr = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!GetSystemPreferredUILanguages flags:0x{:x} num:0x{:x} buf:0x{:x} size:0x{:x}",
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
    stdcall_ret(emu, 4);
}
