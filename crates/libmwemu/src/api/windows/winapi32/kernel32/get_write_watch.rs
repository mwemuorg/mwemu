use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetWriteWatch(emu: &mut emu::Emu) {
    let flags = arg(emu, 0);
    let base_addr = arg(emu, 1) as u64;
    let region_size = arg(emu, 2) as u64;
    let addresses_ptr = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!GetWriteWatch flags:0x{:x} base:0x{:x} size:0x{:x}",
        flags,
        base_addr,
        region_size
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}
