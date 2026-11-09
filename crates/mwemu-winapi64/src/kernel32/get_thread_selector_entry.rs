use crate::emu;

pub fn GetThreadSelectorEntry(emu: &mut emu::Emu) {
    let h_thread = emu.regs().rcx;
    let selector = emu.regs().rdx as u32;
    let entry_ptr = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!GetThreadSelectorEntry h:0x{:x} sel:0x{:x} ptr:0x{:x}",
        h_thread,
        selector,
        entry_ptr
    );
    if entry_ptr != 0 {
        for i in 0..2u64 {
            emu.maps.write_dword(entry_ptr + i * 4, 0);
        }
    }
    emu.regs_mut().rax = 0;
}
