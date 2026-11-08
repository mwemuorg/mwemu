use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CheckForReadOnlyResourceFilter(emu: &mut emu::Emu) {
    let p_exception = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!CheckForReadOnlyResourceFilter pException:0x{:x}",
        p_exception
    );
    let _ = emu.maps.is_mapped(arg(emu, 0) as u64);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}
