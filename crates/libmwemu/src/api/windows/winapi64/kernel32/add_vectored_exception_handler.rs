use crate::emu;

pub fn AddVectoredExceptionHandler(emu: &mut emu::Emu) {
    let p1 = emu.regs().rcx as usize;
    let fptr = emu.regs().rdx as usize;

    log_red!(
        emu,
        "kernel32!AddVectoredExceptionHandler  {} callback: 0x{:x}",
        p1,
        fptr
    );

    emu.set_veh(fptr as u64);

    emu.regs_mut().rax = 0x2c2878;
}
