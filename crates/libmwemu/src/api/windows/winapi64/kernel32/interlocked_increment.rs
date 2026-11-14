use crate::emu;

pub fn InterlockedIncrement(emu: &mut emu::Emu, addr: u64) {
    let addr = emu.regs().rcx;
    let val = emu.maps.read_dword(addr).unwrap_or(0).wrapping_add(1);
    emu.maps.write_dword(addr, val);
    log_red!(emu, "kernel32!InterlockedIncrement 0x{:x} => {}", addr, val);
    emu.regs_mut().rax = val as u64;
}
