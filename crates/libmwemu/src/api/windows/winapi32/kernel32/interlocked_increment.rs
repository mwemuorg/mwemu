use crate::emu;

pub fn InterlockedIncrement(emu: &mut emu::Emu) {
    let addend = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!InterlockedIncrement cannot read addend");

    let prev = emu
        .maps
        .read_dword(addend as u64)
        .expect("kernel32!InterlockedIncrement  error derreferencing addend");

    emu.maps.write_dword(addend as u64, prev + 1);

    log_red!(
        emu,
        "kernel32!InterlockedIncrement 0x{:x} {}->{}",
        addend,
        prev,
        prev + 1
    );

    emu.stack_pop32(false);
    emu.regs_mut().rax = prev as u64 + 1;
}
