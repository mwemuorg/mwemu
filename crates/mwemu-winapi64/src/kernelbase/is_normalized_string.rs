use crate::emu;

pub fn IsNormalizedString(emu: &mut emu::Emu) {
    let norm_form = emu.regs().rcx;
    let str_ptr = emu.regs().rdx;
    let s = if str_ptr != 0 {
        emu.maps.read_wide_string(str_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernelbase!IsNormalizedString form:{} '{}' => true",
        norm_form,
        s
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
