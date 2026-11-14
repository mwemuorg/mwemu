use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn WPUCompleteOverlappedRequest(emu: &mut emu::Emu) {
    let sock = WinApiAbi::arg(emu, 0);
    let overlapped_ptr = WinApiAbi::arg(emu, 1);
    let error = WinApiAbi::arg(emu, 2) as u32;
    let bytes = WinApiAbi::arg(emu, 3) as u32;
    log_red!(
        emu,
        "ws2_32!WPUCompleteOverlappedRequest sock:0x{:x} err:{} bytes:{}",
        sock,
        error,
        bytes
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 5);
}
