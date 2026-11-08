use crate::emu;

pub fn OleConvertIStorageToOLESTREAM(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "ole32!OleConvertIStorageToOLESTREAM arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..1 {}
}
