use crate::emu;

pub fn InterlockedDecrement_api(emu: &mut emu::Emu, addr: u64) {
    let addr = emu.regs().rcx;
    let val = emu.maps.read_dword(addr).unwrap_or(0).wrapping_sub(1);
    emu.maps.write_dword(addr, val);
    log_red!(emu, "kernel32!InterlockedDecrement 0x{:x} => {}", addr, val);
    emu.regs_mut().rax = val as u64;
}
