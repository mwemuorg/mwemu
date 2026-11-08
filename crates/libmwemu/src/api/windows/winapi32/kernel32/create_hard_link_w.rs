use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CreateHardLinkW(emu: &mut emu::Emu) {
    let name_ptr = arg(emu, 0) as u64;
    let param2 = arg(emu, 1) as u64;
    let param3 = arg(emu, 2) as u64;
    let param4 = arg(emu, 3) as u64;

    let name = emu.maps.read_wide_string(name_ptr);
    log_red!(emu, "kernel32!CreateHardLinkW name:'{}'", name);

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
