use crate::emu;

pub fn GetLongPathNameW(emu: &mut emu::Emu) {
    let short_path_ptr = emu.regs().rcx;
    let long_path_ptr = emu.regs().rdx;
    let buff = emu.regs().r8;

    let short = emu.maps.read_wide_string(short_path_ptr);

    log_red!(
        emu,
        "kernel32!GetLongPathNameW  {} {:x}",
        short,
        long_path_ptr
    );

    if long_path_ptr > 0 {
        let mut base = String::from("\\.\\");
        base.push_str(&short);
        emu.maps.write_wide_string(long_path_ptr, &base);
    }

    emu.regs_mut().rax = short.len() as u64;
}
