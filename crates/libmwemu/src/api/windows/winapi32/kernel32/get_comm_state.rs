use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetCommState(emu: &mut emu::Emu) {
    let h_file = arg(emu, 0) as u64;
    let dcb_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!GetCommState h:0x{:x} dcb:0x{:x}",
        h_file,
        dcb_ptr
    );
    if dcb_ptr != 0 {
        emu.maps.write_dword(dcb_ptr, 28);
        emu.maps.write_dword(dcb_ptr + 4, 9600);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
