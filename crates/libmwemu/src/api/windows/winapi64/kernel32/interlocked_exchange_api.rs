use crate::emu;

pub fn InterlockedExchange_api(emu: &mut emu::Emu, addr: u64) {
    let addr = emu.regs().rcx;
    let new_val = emu.regs().rdx as u32;
    let old = emu.maps.read_dword(addr).unwrap_or(0);
    emu.maps.write_dword(addr, new_val);
    log_red!(
        emu,
        "kernel32!InterlockedExchange 0x{:x} {} => {}",
        addr,
        new_val,
        old
    );
    emu.regs_mut().rax = old as u64;
}
