use crate::emu;

pub fn InterlockedCompareExchange(emu: &mut emu::Emu) {
    let destination = emu.regs().rcx;
    let exchange = emu.regs().rdx as u32;
    let comparand = emu.regs().r8 as u32;

    let old_val = emu.maps.read_dword(destination).unwrap_or(0);
    log_red!(
        emu,
        "kernel32!InterlockedCompareExchange dest:0x{:x} exchange:{} comparand:{} old:{}",
        destination,
        exchange,
        comparand,
        old_val
    );

    if old_val == comparand {
        emu.maps.write_dword(destination, exchange);
    }
    emu.regs_mut().rax = old_val as u64;
}
