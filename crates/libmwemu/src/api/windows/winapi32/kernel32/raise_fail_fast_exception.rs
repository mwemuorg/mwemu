use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn RaiseFailFastException(emu: &mut emu::Emu) {
    let exception_record = arg(emu, 0) as u64;
    let context_record = arg(emu, 1) as u64;
    let flags = arg(emu, 2) as u64;

    log_red!(
        emu,
        "kernel32!RaiseFailFastException pExcept:0x{:x} pCtx:0x{:x} flags:0x{:x}",
        exception_record,
        context_record,
        flags
    );
    let first_param = arg(emu, 0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    stdcall_ret(emu, 3);
}
