use crate::emu;

pub fn InterlockedExchange(emu: &mut emu::Emu) {
    let target_ptr = emu.regs().rcx;
    let value = emu.regs().rdx as u32;

    let prev = emu.maps.read_dword(target_ptr).unwrap_or(0);
    emu.maps.write_dword(target_ptr, value);

    log_red!(
        emu,
        "kernel32!InterlockedExchange 0x{:x} old:{} new:{}",
        target_ptr,
        prev,
        value
    );
    emu.regs_mut().rax = prev as u64;
}
