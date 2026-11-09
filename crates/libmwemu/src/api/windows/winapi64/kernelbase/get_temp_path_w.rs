use crate::emu;

pub fn GetTempPathW(emu: &mut emu::Emu) {
    let buf_size = emu.regs().rcx;
    let buf_ptr = emu.regs().rdx;
    let tmp = "C:\\Windows\\Temp\\";
    if buf_ptr != 0 && buf_size > 0 {
        emu.maps.write_wide_string(buf_ptr, tmp);
    }
    log_red!(emu, "kernelbase!GetTempPathW => '{}'", tmp);
    emu.set_last_error(0);
    emu.regs_mut().rax = tmp.len() as u64;
}
