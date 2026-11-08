use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetProcessVersion(emu: &mut emu::Emu) {
    let process_id = arg(emu, 0);
    log_red!(
        emu,
        "kernel32!GetProcessVersion pid:0x{:x} => 0x000A0000",
        process_id
    );
    let _ = emu.maps.is_mapped(arg(emu, 0) as u64);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x000A0000;
    stdcall_ret(emu, 1);
}
