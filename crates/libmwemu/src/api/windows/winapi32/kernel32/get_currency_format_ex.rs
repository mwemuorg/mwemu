use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetCurrencyFormatEx(emu: &mut emu::Emu) {
    let locale_ptr = arg(emu, 0) as u64;
    let flags = arg(emu, 1);
    let value_ptr = arg(emu, 2) as u64;
    let format_ptr = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!GetCurrencyFormatEx locale:0x{:x} flags:0x{:x} val:0x{:x}",
        locale_ptr,
        flags,
        value_ptr
    );
    let out_ptr = arg(emu, 3) as u64;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}
