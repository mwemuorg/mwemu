use crate::emu;

pub fn InterlockedDecrement(emu: &mut emu::Emu) {
    let addend_ptr = emu.regs().rcx;

    let prev = emu.maps.read_dword(addend_ptr).unwrap_or(0);
    let new_val = prev.wrapping_sub(1);
    emu.maps.write_dword(addend_ptr, new_val);

    log_red!(
        emu,
        "kernel32!InterlockedDecrement 0x{:x} {}->{}", addend_ptr, prev, new_val
    );
    emu.regs_mut().rax = new_val as u64;
}
