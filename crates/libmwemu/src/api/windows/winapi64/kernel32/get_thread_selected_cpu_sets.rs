use crate::emu;

pub fn GetThreadSelectedCpuSets(emu: &mut emu::Emu) {
    let h_thread = emu.regs().rcx;
    let cpu_set_ids_ptr = emu.regs().rdx;
    let count = emu.regs().r8 as u32;
    let required_count_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetThreadSelectedCpuSets h:0x{:x} count:{}",
        h_thread,
        count
    );
    if required_count_ptr != 0 {
        emu.maps.write_dword(required_count_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
