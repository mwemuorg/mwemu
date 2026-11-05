use crate::emu;

pub fn SetLocaleInfoA(emu: &mut emu::Emu) {
    let locale = emu.regs().rcx as u32;
    let lc_type = emu.regs().rdx as u32;
    let data_ptr = emu.regs().r8;
    let data = emu.maps.read_string(data_ptr);
    log_red!(
        emu,
        "kernel32!SetLocaleInfoA locale:0x{:x} type:{} data:'{}'",
        locale,
        lc_type,
        data
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
