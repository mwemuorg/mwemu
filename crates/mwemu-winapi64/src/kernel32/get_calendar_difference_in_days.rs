use crate::emu;

pub fn GetCalendarDifferenceInDays(emu: &mut emu::Emu) {
    let st1_ptr = emu.regs().rcx;
    let st2_ptr = emu.regs().rdx;
    let result_ptr = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!GetCalendarDifferenceInDays st1:0x{:x} st2:0x{:x} result:0x{:x}",
        st1_ptr,
        st2_ptr,
        result_ptr
    );
    if result_ptr != 0 {
        emu.maps.write_dword(result_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
