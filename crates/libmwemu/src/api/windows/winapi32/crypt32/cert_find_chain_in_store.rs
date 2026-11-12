use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CertFindChainInStore(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let arg3 = WinApiAbi::arg(emu, 2);
    let arg4 = WinApiAbi::arg(emu, 3);
    let arg5 = WinApiAbi::arg(emu, 4);
    let arg6 = WinApiAbi::arg(emu, 5);
    for _ in 0..6u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "crypt32!CertFindChainInStore arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x} arg6:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4,
        arg5,
        arg6
    );
    emu.regs_mut().rax = helper::handler_create("CertFindChainInStore");
}
