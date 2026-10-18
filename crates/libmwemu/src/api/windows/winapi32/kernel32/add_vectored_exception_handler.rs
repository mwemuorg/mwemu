use crate::emu;

pub fn AddVectoredExceptionHandler(emu: &mut emu::Emu) {
    let first = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!AddVectoredExceptionHandler: error reading first") as u64;
    let fptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!AddVectoredExceptionHandler: error reading fptr") as u64;

    log_red!(
        emu,
        "kernel32!AddVectoredExceptionHandler  first={} callback: 0x{:x}",
        first,
        fptr
    );

    emu.add_veh(first != 0, fptr);

    emu.regs_mut().rax = 0x2c2878;
    emu.stack_pop32(false);
    emu.stack_pop32(false);
}
