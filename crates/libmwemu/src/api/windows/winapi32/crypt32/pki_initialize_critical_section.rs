use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn PkiInitializeCriticalSection(emu: &mut emu::Emu) {
    let addr = arg(emu, 0);
    let flags = arg(emu, 1);

    log_red!(
        emu,
        "crypt32!PkiInitializeCriticalSection flags: 0x{:x} addr: 0x{:x}",
        flags,
        addr
    );

    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
