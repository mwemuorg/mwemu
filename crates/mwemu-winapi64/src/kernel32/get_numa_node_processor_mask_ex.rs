use crate::emu;

pub fn GetNumaNodeProcessorMaskEx(emu: &mut emu::Emu) {
    let node = emu.regs().rcx as u16;
    let affinity_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetNumaNodeProcessorMaskEx node:{} ptr:0x{:x}",
        node,
        affinity_ptr
    );
    if affinity_ptr != 0 {
        emu.maps.write_qword(affinity_ptr, 0xF);
        emu.maps.write_word(affinity_ptr + 8, 0);
        for i in 0..3u64 {
            emu.maps.write_word(affinity_ptr + 10 + i * 2, 0);
        }
    }
    emu.regs_mut().rax = 1;
}
