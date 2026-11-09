use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn AdjustTokenPrivileges(emu: &mut emu::Emu) {
    let token = arg(emu, 0);
    let disable_all = arg(emu, 1);
    log_red!(
        emu,
        "advapi32!AdjustTokenPrivileges token=0x{:x} disableAll={}",
        token,
        disable_all
    );
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 6);
}
