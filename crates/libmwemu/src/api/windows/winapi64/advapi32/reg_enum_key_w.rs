use crate::emu;

pub fn RegEnumKeyW(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let index = emu.regs().rdx;
    log_red!(
        emu,
        "advapi32!RegEnumKeyW hkey:0x{:x} index:{}",
        hkey,
        index
    );
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 259;
    for _ in 0..8 {}
}
