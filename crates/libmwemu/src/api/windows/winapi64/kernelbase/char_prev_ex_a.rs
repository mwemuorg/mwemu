use crate::emu;

pub fn CharPrevExA(emu: &mut emu::Emu) {
    let _codepage = emu.regs().rcx;
    let start_ptr = emu.regs().rdx;
    let current_ptr = emu.regs().r8;
    let _flags = emu.regs().r9;
    let result = if current_ptr <= start_ptr {
        start_ptr
    } else {
        current_ptr - 1
    };
    log_red!(
        emu,
        "kernelbase!CharPrevExA start:0x{:x} cur:0x{:x} => 0x{:x}",
        start_ptr,
        current_ptr,
        result
    );
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.regs_mut().rax = result;
}
