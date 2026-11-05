use crate::emu;

pub fn InterlockedExchangeAdd(emu: &mut emu::Emu) {
    let addend_ptr = emu.regs().rcx;
    let value = emu.regs().rdx as i32;

    let prev = emu.maps.read_dword(addend_ptr).unwrap_or(0);
    let new_val = (prev as i32).wrapping_add(value) as u32;
    emu.maps.write_dword(addend_ptr, new_val);

    log_red!(
        emu,
        "kernel32!InterlockedExchangeAdd 0x{:x} old:{} +{} = {}",
        addend_ptr,
        prev,
        value,
        new_val
    );
    emu.regs_mut().rax = prev as u64;
}
