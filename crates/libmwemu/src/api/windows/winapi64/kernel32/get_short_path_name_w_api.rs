use crate::emu;

pub fn GetShortPathNameW_api(emu: &mut emu::Emu) {
    let long = emu.regs().rcx;
    let short = emu.regs().rdx;
    let buf_len = emu.regs().r8 as u32;
    let path = emu.maps.read_wide_string(long);
    log_red!(emu, "kernel32!GetShortPathNameW '{}'", path);
    if short != 0 && buf_len > path.len() as u32 {
        emu.maps.write_wide_string(short, &path);
    }
    emu.regs_mut().rax = path.len() as u64;
}
