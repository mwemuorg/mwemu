use crate::emu;

pub fn GetNumaNodeProcessorMask2(emu: &mut emu::Emu) {
    let node = emu.regs().rcx as u16;
    let masks_ptr = emu.regs().rdx;
    let mask_count = emu.regs().r8 as u32;
    let req_count_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetNumaNodeProcessorMask2 node:{} masks:0x{:x} count:{} req:0x{:x}",
        node,
        masks_ptr,
        mask_count,
        req_count_ptr
    );
    if req_count_ptr != 0 {
        emu.maps.write_dword(req_count_ptr, 1);
    }
    if masks_ptr != 0 && mask_count >= 1 {
        emu.maps.write_qword(masks_ptr, 0xF);
        emu.maps.write_word(masks_ptr + 8, 0);
    }
    emu.regs_mut().rax = 1;
}
