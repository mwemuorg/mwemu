use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetEraNameCountedString(emu: &mut emu::Emu) {
    let locale_ptr = arg(emu, 0) as u64;
    let calendar = arg(emu, 1);
    let era = arg(emu, 2);
    let result_ptr = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!GetEraNameCountedString cal:{} era:{} result:0x{:x}",
        calendar,
        era,
        result_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}
