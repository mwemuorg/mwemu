use crate::emu;

pub fn VirtualProtect(emu: &mut emu::Emu, addr: u64) {
    let addr = emu.regs().rcx;
    let size = emu.regs().rdx;
    let new_protect = emu.regs().r8;
    let old_protect_ptr = emu.regs().r9;
    if old_protect_ptr != 0 {
        emu.maps.write_dword(old_protect_ptr, 0x40); // PAGE_EXECUTE_READWRITE
    }
    log_red!(
        emu,
        "kernelbase!VirtualProtect addr:0x{:x} size:0x{:x} prot:0x{:x}",
        addr,
        size,
        new_protect
    );
    emu.regs_mut().rax = 1;
}
