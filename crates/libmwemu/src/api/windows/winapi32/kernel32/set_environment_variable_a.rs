use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn SetEnvironmentVariableA(emu: &mut emu::Emu) {
    let name_ptr = arg(emu, 0) as u64;
    let value_ptr = arg(emu, 1) as u64;
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
    stdcall_ret(emu, 2);
}
