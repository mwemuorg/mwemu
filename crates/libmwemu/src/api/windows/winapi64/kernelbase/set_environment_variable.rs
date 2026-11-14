use crate::emu;
use crate::winapi::common::strings;

pub fn SetEnvironmentVariableA(emu: &mut emu::Emu) {
    set_environment_variable(emu, "SetEnvironmentVariableA", false);
}

fn set_environment_variable(emu: &mut emu::Emu, api: &str, wide: bool) {
    let name_ptr = emu.regs().rcx;
    let value_ptr = emu.regs().rdx;
    let name = strings::read_text(emu, name_ptr, wide);
    let value = if value_ptr != 0 {
        strings::read_text(emu, value_ptr, wide)
    } else {
        String::from("(null)")
    };
    log_red!(emu, "kernelbase!{api} '{}' = '{}'", name, value);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SetEnvironmentVariableW(emu: &mut emu::Emu) {
    set_environment_variable(emu, "SetEnvironmentVariableW", true);
}
