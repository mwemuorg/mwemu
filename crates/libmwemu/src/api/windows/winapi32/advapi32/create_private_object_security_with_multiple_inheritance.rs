use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreatePrivateObjectSecurityWithMultipleInheritance(emu: &mut emu::Emu) {
    let handle = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    log_red!(
        emu,
        "advapi32!CreatePrivateObjectSecurityWithMultipleInheritance h:0x{:x} arg2:0x{:x}",
        handle,
        arg2
    );
    stdcall_ret(emu, 9);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
