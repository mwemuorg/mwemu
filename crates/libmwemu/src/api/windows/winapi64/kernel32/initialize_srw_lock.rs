use crate::emu;

pub fn InitializeSRWLock(emu: &mut emu::Emu) {
    let lock_ptr = emu.regs().rcx;

    log_red!(emu, "kernel32!InitializeSRWLock ptr: 0x{:x}", lock_ptr);

    emu.maps.write_qword(lock_ptr, 0);
}
