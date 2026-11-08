use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CommConfigDialogW(emu: &mut emu::Emu) {
    let param1 = arg(emu, 0) as u64;
    let param2 = arg(emu, 1) as u64;
    let param3 = arg(emu, 2) as u64;
    let param4 = arg(emu, 3) as u64;

    log_red!(
        emu,
        "kernel32!CommConfigDialogW p1:0x{:x} p2:0x{:x} p3:0x{:x} p4:0x{:x}",
        param1,
        param2,
        param3,
        param4
    );
    let first_param = arg(emu, 0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
