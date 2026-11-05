use crate::emu;

pub fn GetProcessGroupAffinity(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    let count_ptr = emu.regs().rdx;
    let array_ptr = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!GetProcessGroupAffinity h:0x{:x} count:0x{:x} array:0x{:x}",
        h_process,
        count_ptr,
        array_ptr
    );
    if count_ptr != 0 {
        emu.maps.write_word(count_ptr, 1);
    }
    if array_ptr != 0 {
        emu.maps.write_word(array_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
