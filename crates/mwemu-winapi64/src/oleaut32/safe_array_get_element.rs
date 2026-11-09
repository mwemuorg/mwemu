use crate::emu;

pub fn SafeArrayGetElement(emu: &mut emu::Emu) {
    let sa_ptr = emu.regs().rcx;
    let index = emu.regs().rdx;
    let out_ptr = emu.regs().r8;
    log_red!(
        emu,
        "oleaut32!SafeArrayGetElement sa:0x{:x} idx:0x{:x}",
        sa_ptr,
        index
    );
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
