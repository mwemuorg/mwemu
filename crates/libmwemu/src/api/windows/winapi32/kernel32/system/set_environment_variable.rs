use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn SetEnvironmentVariableA(emu: &mut emu::Emu) {
    set_environment_variable(emu, "SetEnvironmentVariableA", false);
}

fn set_environment_variable(emu: &mut emu::Emu, api: &str, wide: bool) {
    let name_ptr = arg(emu, 0) as u64;
    let value_ptr = arg(emu, 1) as u64;
    let name = strings::read_text(emu, name_ptr, wide);
    let value = if value_ptr != 0 {
        strings::read_text(emu, value_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "kernel32!{api} `{}`=`{}` (stub)", name, value);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

pub fn SetEnvironmentVariableW(emu: &mut emu::Emu) {
    set_environment_variable(emu, "SetEnvironmentVariableW", true);
}
