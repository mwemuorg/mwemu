use crate::emu;
use crate::winapi::helper;

pub fn WSASendTo(emu: &mut emu::Emu) {
    let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let buffers_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let buffer_count = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let bytes_sent_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
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
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
}
