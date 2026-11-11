use crate::emu;

pub(super) fn PeekMessageA(emu: &mut emu::Emu) {
    let msg_ptr = emu.regs().rcx;
    let hwnd = emu.regs().rdx;
    let msg_filter_min = emu.regs().r8;
    let msg_filter_max = emu.regs().r9;
    let remove_msg = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    if msg_ptr != 0 {
        for i in 0..7u64 {
            emu.maps.write_qword(msg_ptr + i * 8, 0);
        }
    }
    log_red!(
        emu,
        "user32!PeekMessageA msg:0x{:x} hwnd:0x{:x} min:{} max:{} rm:0x{:x}",
        msg_ptr,
        hwnd,
        msg_filter_min,
        msg_filter_max,
        remove_msg
    );
    emu.regs_mut().rax = 0;
}

pub(super) fn PeekMessageW(emu: &mut emu::Emu) {
    let msg_ptr = emu.regs().rcx;
    let hwnd = emu.regs().rdx;
    let msg_filter_min = emu.regs().r8;
    let msg_filter_max = emu.regs().r9;
    let remove_msg = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    if msg_ptr != 0 {
        for i in 0..7u64 {
            emu.maps.write_qword(msg_ptr + i * 8, 0);
        }
    }
    log_red!(
        emu,
        "user32!PeekMessageW msg:0x{:x} hwnd:0x{:x} min:{} max:{} rm:0x{:x}",
        msg_ptr,
        hwnd,
        msg_filter_min,
        msg_filter_max,
        remove_msg
    );
    emu.regs_mut().rax = 0;
}
