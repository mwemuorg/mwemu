use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetConsoleTitleA(emu: &mut emu::Emu) {
    let buff_ptr = arg(emu, 0) as u64;
    let size = arg(emu, 1);

    log_red!(emu, "kernel32!GetConsoleTitleA size:{}", size);
    let out_ptr = arg(emu, 1) as u64;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }

    let title = "Windows Console";
    if buff_ptr != 0 && size > 0 {
        emu.maps.write_string(buff_ptr, title);
    }

    emu.regs_mut().rax = title.len() as u64;
    stdcall_ret(emu, 2);
}
