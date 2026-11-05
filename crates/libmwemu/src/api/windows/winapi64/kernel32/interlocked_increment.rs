use crate::emu;

pub fn InterlockedIncrement(emu: &mut emu::Emu) {
    let addend = emu.regs().rcx as u32;

    let prev = emu.maps.read_dword(addend as u64).unwrap_or(0);

    emu.maps.write_dword(addend as u64, prev + 1);

    log_red!(
        emu,
        "kernel32!InterlockedIncrement 0x{:x} {}->{}",
        addend,
        prev,
        prev + 1
    );
    emu.regs_mut().rax = prev as u64 + 1;
}
