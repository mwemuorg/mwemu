use crate::emu;

pub fn GetCalendarInfoEx(emu: &mut emu::Emu) {
    let locale_ptr = emu.regs().rcx;
    let calendar = emu.regs().rdx as u32;
    let cal_type = emu.regs().r8 as u32;
    let data_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetCalendarInfoEx locale:0x{:x} cal:{} type:{} data:0x{:x}",
        locale_ptr,
        calendar,
        cal_type,
        data_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
