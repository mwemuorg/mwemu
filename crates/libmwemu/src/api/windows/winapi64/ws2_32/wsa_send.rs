use crate::emu;

pub(super) fn WSASend(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let buffers = emu.regs().rdx;
    let count = emu.regs().r8 as u32;
    let bytes_sent_ptr = emu.regs().r9;
    log_red!(emu, "ws2_32!WSASend sock:0x{:x} bufs:{} ", sock, count);
    let total: u32 = if buffers != 0 && count > 0 {
        let mut sum = 0u32;
        for i in 0..count as u64 {
            let len = emu.maps.read_dword(buffers + i * 16).unwrap_or(0);
            sum += len;
        }
        sum
    } else {
        0
    };
    if bytes_sent_ptr != 0 {
        emu.maps.write_dword(bytes_sent_ptr, total);
    }
    emu.regs_mut().rax = 0;
}
