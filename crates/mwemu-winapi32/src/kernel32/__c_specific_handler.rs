use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn __C_specific_handler(emu: &mut emu::Emu) {
    let exception_record = arg(emu, 0) as u64;
    let establisher_frame = arg(emu, 1) as u64;

    log_red!(
        emu,
        "kernel32!__C_specific_handler excRec:0x{:x} frame:0x{:x}",
        exception_record,
        establisher_frame
    );
    let first_param = arg(emu, 0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
