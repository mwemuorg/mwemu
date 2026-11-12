use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub(super) fn PeekMessageA(emu: &mut emu::Emu) {
    peek_message(emu, "PeekMessageA");
}

fn peek_message(emu: &mut emu::Emu, api: &str) {
    let msg_ptr = emu.regs().rcx;
    let hwnd = emu.regs().rdx;
    let msg_filter_min = emu.regs().r8;
    let msg_filter_max = emu.regs().r9;
    let remove_msg = WinApiAbi::arg(emu, 4);
    if msg_ptr != 0 {
        for i in 0..7u64 {
            emu.maps.write_qword(msg_ptr + i * 8, 0);
        }
    }
    log_red!(
        emu,
        "user32!{api} msg:0x{:x} hwnd:0x{:x} min:{} max:{} rm:0x{:x}",
        msg_ptr,
        hwnd,
        msg_filter_min,
        msg_filter_max,
        remove_msg
    );
    emu.regs_mut().rax = 0;
}

pub(super) fn PeekMessageW(emu: &mut emu::Emu) {
    peek_message(emu, "PeekMessageW");
}
