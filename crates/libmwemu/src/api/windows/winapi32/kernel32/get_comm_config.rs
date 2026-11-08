use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetCommConfig(emu: &mut emu::Emu) {
    let h_comm_dev = arg(emu, 0) as u64;
    let config_ptr = arg(emu, 1) as u64;
    let size_ptr = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!GetCommConfig h:0x{:x} config:0x{:x} size:0x{:x}",
        h_comm_dev,
        config_ptr,
        size_ptr
    );
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, 52);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
