use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn WaitNamedPipeW(emu: &mut emu::Emu) {
    let name_ptr = arg(emu, 0) as u64;
    let timeout = arg(emu, 1) as u64;

    let name = if name_ptr != 0 {
        emu.maps.read_wide_string(name_ptr)
    } else {
        String::new()
    };

    log_red!(
        emu,
        "kernel32!WaitNamedPipeW '{}' timeout:{}",
        name,
        timeout
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
