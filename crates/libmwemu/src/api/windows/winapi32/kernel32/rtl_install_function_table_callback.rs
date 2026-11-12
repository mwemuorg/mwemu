use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn RtlInstallFunctionTableCallback(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = WinApiAbi::arg64(emu, 2);
    log_red!(
        emu,
        "kernel32!RtlInstallFunctionTableCallback arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    stdcall_ret(emu, 8);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
