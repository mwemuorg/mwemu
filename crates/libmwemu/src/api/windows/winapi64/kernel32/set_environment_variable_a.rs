use crate::emu;

pub fn SetEnvironmentVariableA(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().rcx;
    let value_ptr = emu.regs().rdx;

    let name = emu.maps.read_string(name_ptr);
    let value = if value_ptr != 0 {
        emu.maps.read_string(value_ptr)
    } else {
        String::new()
    };

    log_red!(
        emu,
        "kernel32!SetEnvironmentVariableA `{}`=`{}` (stub)",
        name,
        value
    );

    emu.regs_mut().rax = 1;
}
