use crate::emu;

pub fn SetLocaleInfoW(emu: &mut emu::Emu) {
    let locale = emu.regs().rcx;
    let lc_type = emu.regs().rdx;
    let data_ptr = emu.regs().r8;

    let data = if data_ptr != 0 {
        emu.maps.read_wide_string(data_ptr)
    } else {
        String::new()
    };

    log_red!(
        emu,
        "kernel32!SetLocaleInfoW locale:0x{:x} type:0x{:x} '{}'",
        locale,
        lc_type,
        data
    );
    emu.regs_mut().rax = 1;
}
