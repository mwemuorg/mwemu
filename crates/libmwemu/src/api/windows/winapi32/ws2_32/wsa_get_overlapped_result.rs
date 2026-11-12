use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSAGetOverlappedResult(emu: &mut emu::Emu) {
    let sock = WinApiAbi::arg(emu, 0);
    let overlapped_ptr = WinApiAbi::arg(emu, 1);
    let transfer_ptr = WinApiAbi::arg(emu, 2);
    let wait = WinApiAbi::arg(emu, 3) as u32;
    log_red!(
        emu,
        "ws2_32!WSAGetOverlappedResult sock:0x{:x} wait:{}",
        sock,
        wait
    );
    if transfer_ptr != 0 {
        emu.maps.write_dword(transfer_ptr, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}
