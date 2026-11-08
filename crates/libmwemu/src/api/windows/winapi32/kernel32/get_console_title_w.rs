use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetConsoleTitleW(emu: &mut emu::Emu) {
    let buff_ptr = arg(emu, 0) as u64;
    let size = arg(emu, 1);

    log_red!(emu, "kernel32!GetConsoleTitleW size:{}", size);
    let out_ptr = arg(emu, 1) as u64;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }

    let title = "Windows Console";
    if buff_ptr != 0 && size > 0 {
        emu.maps.write_wide_string(buff_ptr, title);
    }

    emu.regs_mut().rax = title.chars().count() as u64;
    stdcall_ret(emu, 2);
}
