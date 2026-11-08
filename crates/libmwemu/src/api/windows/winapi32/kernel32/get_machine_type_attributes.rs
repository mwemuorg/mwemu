use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetMachineTypeAttributes(emu: &mut emu::Emu) {
    let machine = arg(emu, 0) as u64 as u16;
    let attrs_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!GetMachineTypeAttributes machine:0x{:x} attrs:0x{:x}",
        machine,
        attrs_ptr
    );
    if attrs_ptr != 0 {
        emu.maps.write_dword(attrs_ptr, 0x10);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}
