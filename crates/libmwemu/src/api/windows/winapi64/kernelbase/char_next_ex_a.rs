use crate::emu;

pub fn CharNextExA(emu: &mut emu::Emu) {
    let _codepage = emu.regs().rcx;
    let ptr = emu.regs().rdx;
    let _flags = emu.regs().r8;
    let b = if ptr != 0 {
        emu.maps.read_byte(ptr).unwrap_or(0)
    } else {
        0
    };
    log_red!(emu, "kernelbase!CharNextExA ptr:0x{:x}", ptr);
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = if b == 0 { ptr } else { ptr + 1 };
}
