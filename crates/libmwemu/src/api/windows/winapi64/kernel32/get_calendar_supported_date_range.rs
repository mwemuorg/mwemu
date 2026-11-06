use crate::emu;

pub fn GetCalendarSupportedDateRange(emu: &mut emu::Emu) {
    let calendar = emu.regs().rcx as u32;
    let begin_ptr = emu.regs().rdx;
    let end_ptr = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!GetCalendarSupportedDateRange cal:{} begin:0x{:x} end:0x{:x}",
        calendar,
        begin_ptr,
        end_ptr
    );
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
