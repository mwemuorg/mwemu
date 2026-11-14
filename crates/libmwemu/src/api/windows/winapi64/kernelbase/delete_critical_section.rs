use crate::emu;

pub fn DeleteCriticalSection(emu: &mut emu::Emu) {
    let cs_ptr = emu.regs().rcx;
    if cs_ptr != 0 {
        emu.maps.write_qword(cs_ptr, 0);
    }
    log_red!(emu, "kernelbase!DeleteCriticalSection 0x{:x}", cs_ptr);
}
