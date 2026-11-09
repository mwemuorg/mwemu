use crate::emu;

pub fn GetShortPathNameW(emu: &mut emu::Emu) {
    let long_ptr = emu.regs().rcx;
    let short_ptr = emu.regs().rdx;
    let buf_size = emu.regs().r8;
    let path = if long_ptr != 0 {
        emu.maps.read_wide_string(long_ptr)
    } else {
        String::new()
    };
    let char_count = path.encode_utf16().count() + 1;
    if short_ptr != 0 && buf_size > 0 {
        emu.maps.write_wide_string(short_ptr, &path);
    }
    log_red!(
        emu,
        "kernelbase!GetShortPathNameW '{}' => len:{}",
        path,
        char_count
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = char_count as u64;
}
