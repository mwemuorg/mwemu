use crate::emu;

pub fn SetEnvironmentVariableW(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().rcx;
    let value_ptr = emu.regs().rdx;
    let name = emu.maps.read_wide_string(name_ptr);
    let value = if value_ptr != 0 {
        emu.maps.read_wide_string(value_ptr)
    } else {
        String::from("(null)")
    };
    log_red!(
        emu,
        "kernelbase!SetEnvironmentVariableW '{}' = '{}'",
        name,
        value
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
