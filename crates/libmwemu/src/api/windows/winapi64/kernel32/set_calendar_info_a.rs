use crate::emu;

pub fn SetCalendarInfoA(emu: &mut emu::Emu) {
    let locale = emu.regs().rcx as u32;
    let calendar = emu.regs().rdx as u32;
    let cal_type = emu.regs().r8 as u32;
    let data_ptr = emu.regs().r9;
    let data = emu.maps.read_string(data_ptr);
    log_red!(
        emu,
        "kernel32!SetCalendarInfoA locale:0x{:x} cal:{} type:{} data:'{}'",
        locale,
        calendar,
        cal_type,
        data
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
