use crate::emu;

pub fn GetThreadIdealProcessorEx(emu: &mut emu::Emu) {
    let h_thread = emu.regs().rcx;
    let proc_num_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetThreadIdealProcessorEx h:0x{:x} ptr:0x{:x}",
        h_thread,
        proc_num_ptr
    );
    if proc_num_ptr != 0 {
        emu.maps.write_word(proc_num_ptr, 0);
        emu.maps.write_byte(proc_num_ptr + 2, 0);
        emu.maps.write_byte(proc_num_ptr + 3, 0);
    }
    emu.regs_mut().rax = 1;
}
