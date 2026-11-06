use crate::emu;

pub fn GetIoRingInfo(emu: &mut emu::Emu) {
    let io_ring = emu.regs().rcx;
    let info_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetIoRingInfo ring:0x{:x} info:0x{:x}",
        io_ring,
        info_ptr
    );
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x80070057u64;
}
