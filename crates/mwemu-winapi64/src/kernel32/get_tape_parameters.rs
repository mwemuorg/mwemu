use crate::emu;

pub fn GetTapeParameters(emu: &mut emu::Emu) {
    let h_device = emu.regs().rcx;
    let operation = emu.regs().rdx as u32;
    let size_ptr = emu.regs().r8;
    let info_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetTapeParameters h:0x{:x} op:{}",
        h_device,
        operation
    );
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
