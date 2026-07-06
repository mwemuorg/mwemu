use crate::emu;

pub fn GetLongPathNameW(emu: &mut emu::Emu) {
    let short_path_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!GetLongPathNameW: error reading param") as u64;
    let long_path_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!GetLongPathNameW: error reading param") as u64;
    let buff = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!GetLongPathNameW: error reading param") as u64;

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

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
