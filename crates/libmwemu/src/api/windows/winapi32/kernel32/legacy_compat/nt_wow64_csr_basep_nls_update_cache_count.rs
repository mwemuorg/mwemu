use crate::emu;
use crate::windows::constants::errors::*;

pub fn NtWow64CsrBasepNlsUpdateCacheCount(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!NtWow64CsrBasepNlsUpdateCacheCount");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
