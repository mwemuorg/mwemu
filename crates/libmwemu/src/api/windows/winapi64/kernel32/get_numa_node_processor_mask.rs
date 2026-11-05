use crate::emu;

pub fn GetNumaNodeProcessorMask(emu: &mut emu::Emu) {
    let node = emu.regs().rcx as u8;
    let mask_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetNumaNodeProcessorMask node:{} ptr:0x{:x}",
        node,
        mask_ptr
    );
    if mask_ptr != 0 {
        emu.maps.write_qword(mask_ptr, 0xF);
    }
    emu.regs_mut().rax = 1;
}
