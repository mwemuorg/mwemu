use crate::emu;

pub fn GetLongPathNameA(emu: &mut emu::Emu) {
    let short_ptr = emu.regs().rcx;
    let long_ptr = emu.regs().rdx;
    let buf_size = emu.regs().r8;
    let path = if short_ptr != 0 {
        emu.maps.read_string(short_ptr)
    } else {
        String::new()
    };
    if long_ptr != 0 && buf_size > 0 {
        emu.maps.write_string(long_ptr, &path);
    }
    log_red!(
        emu,
        "kernelbase!GetLongPathNameA '{}' => len:{}",
        path,
        path.len() + 1
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = (path.len() + 1) as u64;
}
