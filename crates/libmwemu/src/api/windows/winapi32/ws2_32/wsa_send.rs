use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn WSASend(emu: &mut emu::Emu) {
    let sock = arg(emu, 0) as u64;
    let buffers = arg(emu, 1) as u64;
    let count = arg(emu, 2);
    let bytes_sent_ptr = arg(emu, 3) as u64;

    log_red!(emu, "ws2_32!WSASend sock:0x{:x} bufs:{}", sock, count);

    let total: u32 = if buffers != 0 && count > 0 {
        let mut sum = 0u32;
        for i in 0..count as u64 {
            let len = emu.maps.read_dword(buffers + i * 8).unwrap_or(0);
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
    stdcall_ret(emu, 7);
}
