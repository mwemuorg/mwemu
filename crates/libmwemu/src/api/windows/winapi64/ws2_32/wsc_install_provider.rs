use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn WSCInstallProvider(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    let arg5 = WinApiAbi::arg(emu, 4);
    log_red!(
        emu,
        "ws2_32!WSCInstallProvider arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    let handle = helper::handler_create("WSCInstallProvider");
    let out_ptr = WinApiAbi::arg(emu, 4);
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, handle as u32);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
