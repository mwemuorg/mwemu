use crate::emu;

pub fn VirtualLock(emu: &mut emu::Emu) {
    let lp_address = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!VirtualLock cannot read lp_address") as u64;
    let dw_size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!VirtualLock cannot read dw_size") as u64;

    log_red!(
        emu,
        "kernel32!VirtualLock addr: 0x{:x} size: 0x{:x}",
        lp_address,
        dw_size
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    // Return TRUE
    emu.regs_mut().rax = 1;
}
