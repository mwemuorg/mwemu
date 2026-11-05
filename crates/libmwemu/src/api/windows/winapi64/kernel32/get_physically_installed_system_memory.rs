use crate::emu;

pub fn GetPhysicallyInstalledSystemMemory(emu: &mut emu::Emu) {
    let mem_ptr = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!GetPhysicallyInstalledSystemMemory ptr:0x{:x}",
        mem_ptr
    );
    if mem_ptr != 0 {
        emu.maps.write_qword(mem_ptr, 8 * 1024 * 1024);
    }
    emu.regs_mut().rax = 1;
}
