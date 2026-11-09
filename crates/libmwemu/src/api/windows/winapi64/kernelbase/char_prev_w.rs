use crate::emu;

pub fn CharPrevW(emu: &mut emu::Emu) {
    let start_ptr = emu.regs().rcx;
    let current_ptr = emu.regs().rdx;
    let result = if current_ptr <= start_ptr {
        start_ptr
    } else {
        current_ptr - 2
    };
    log_red!(
        emu,
        "kernelbase!CharPrevW start:0x{:x} cur:0x{:x} => 0x{:x}",
        start_ptr,
        current_ptr,
        result
    );
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.regs_mut().rax = result;
}
