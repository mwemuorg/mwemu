use crate::emu;

pub fn GetLongPathNameW_api(emu: &mut emu::Emu) {
    let short = emu.regs().rcx;
    let long = emu.regs().rdx;
    let buf_len = emu.regs().r8 as u32;
    let path = emu.maps.read_wide_string(short);
    log_red!(emu, "kernel32!GetLongPathNameW '{}'", path);
    if long != 0 && buf_len > path.len() as u32 {
        emu.maps.write_wide_string(long, &path);
    }
    emu.regs_mut().rax = path.len() as u64;
}
