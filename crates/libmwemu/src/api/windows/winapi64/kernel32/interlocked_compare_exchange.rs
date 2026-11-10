use crate::emu;

pub fn InterlockedCompareExchange(emu: &mut emu::Emu, addr: u64) {
    let addr = emu.regs().rcx;
    let exchange = emu.regs().rdx as u32;
    let comparand = emu.regs().r8 as u32;
    let current = emu.maps.read_dword(addr).unwrap_or(0);
    if current == comparand {
        emu.maps.write_dword(addr, exchange);
    }
    log_red!(
        emu,
        "kernel32!InterlockedCompareExchange 0x{:x} exch:{} cmp:{} cur:{}",
        addr,
        exchange,
        comparand,
        current
    );
    emu.regs_mut().rax = current as u64;
}
