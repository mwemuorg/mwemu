use crate::emu;

pub fn AddVectoredExceptionHandler(emu: &mut emu::Emu) {
    let first = emu.regs().rcx;
    let fptr = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!AddVectoredExceptionHandler  first={} callback: 0x{:x}",
        first,
        fptr
    );

    emu.add_veh(first != 0, fptr);

    emu.regs_mut().rax = 0x2c2878;
}
