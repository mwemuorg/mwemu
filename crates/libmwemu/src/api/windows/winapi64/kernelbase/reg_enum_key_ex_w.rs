use crate::emu;

pub fn RegEnumKeyExW(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let index = emu.regs().rdx;
    let name_ptr = emu.regs().r8;
    let name_len_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernelbase!RegEnumKeyExW hkey:0x{:x} index:{}",
        hkey,
        index
    );
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 259;
}
