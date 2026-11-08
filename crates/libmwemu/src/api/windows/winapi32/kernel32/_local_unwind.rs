use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn _local_unwind(emu: &mut emu::Emu) {
    let frame = arg(emu, 0) as u64;
    let target = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!_local_unwind frame:0x{:x} target:0x{:x}",
        frame,
        target
    );
    let first_param = arg(emu, 0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}
