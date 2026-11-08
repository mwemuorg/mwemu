use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetCachedSigningLevel(emu: &mut emu::Emu) {
    let h_file = arg(emu, 0) as u64;
    let flags_ptr = arg(emu, 1) as u64;
    let signing_level_ptr = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!GetCachedSigningLevel h:0x{:x} flags:0x{:x} level:0x{:x}",
        h_file,
        flags_ptr,
        signing_level_ptr
    );
    let out_ptr = arg(emu, 2) as u64;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    if signing_level_ptr != 0 {
        emu.maps.write_byte(signing_level_ptr, 0);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}
