use crate::emu;

pub fn GetShortPathNameA_api(emu: &mut emu::Emu) {
    let long = emu.regs().rcx;
    let short = emu.regs().rdx;
    let buf_len = emu.regs().r8 as u32;
    let path = emu.maps.read_string(long);
    log_red!(emu, "kernel32!GetShortPathNameA '{}'", path);
    if short != 0 && buf_len > path.len() as u32 {
        emu.maps.write_string(short, &path);
    }
    emu.regs_mut().rax = path.len() as u64;
}
