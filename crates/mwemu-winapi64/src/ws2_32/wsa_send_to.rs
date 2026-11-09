use crate::emu;
use crate::winapi::helper;

pub fn WSASendTo(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let buffers_ptr = emu.regs().rdx;
    let buffer_count = emu.regs().r8 as u32;
    let bytes_sent_ptr = emu.regs().r9;
    log_red!(
        emu,
        "ws2_32!WSASendTo sock:0x{:x} buffers:{}",
        sock,
        buffer_count
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        if buffers_ptr != 0 && buffer_count > 0 {
            let buf_len = emu.maps.read_dword(buffers_ptr).unwrap_or(0);
            if bytes_sent_ptr != 0 {
                emu.maps.write_dword(bytes_sent_ptr, buf_len);
            }
        }
        emu.regs_mut().rax = 0;
    }
}
