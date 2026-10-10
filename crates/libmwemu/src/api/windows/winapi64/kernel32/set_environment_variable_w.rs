use crate::emu;

pub fn SetEnvironmentVariableW(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().rcx;
    let value_ptr = emu.regs().rdx;

    let name = emu.maps.read_wide_string(name_ptr);
    let value = if value_ptr != 0 {
        emu.maps.read_wide_string(value_ptr)
    } else {
        String::new()
    };

    log_red!(
        emu,
        "kernel32!SetEnvironmentVariableW `{}`=`{}` (stub)",
        name,
        value
    );

    emu.regs_mut().rax = 1;
}
