use crate::emu;

pub fn GetLocalManagedApplicationData(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "advapi32!GetLocalManagedApplicationData h:0x{:x} arg2:0x{:x}",
        handle,
        arg2
    );
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    for _ in 0..2 {}
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
