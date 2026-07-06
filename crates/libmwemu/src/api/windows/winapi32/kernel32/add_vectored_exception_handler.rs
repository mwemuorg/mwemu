use crate::emu;

pub fn AddVectoredExceptionHandler(emu: &mut emu::Emu) {
    let p1 = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!AddVectoredExceptionHandler: error reading p1") as u64;
    let fptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!AddVectoredExceptionHandler: error reading fptr") as u64;

    log_red!(
        emu,
        "kernel32!AddVectoredExceptionHandler  {} callback: 0x{:x}",
        p1,
        fptr
    );

    emu.set_veh(fptr);

    emu.regs_mut().rax = 0x2c2878;
    emu.stack_pop32(false);
    emu.stack_pop32(false);
}
