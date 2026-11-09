use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn select(emu: &mut emu::Emu) {
    let nfds = arg(emu, 0);
    let readfds = arg(emu, 1) as u64;
    let writefds = arg(emu, 2) as u64;
    let exceptfds = arg(emu, 3) as u64;
    let timeout_ptr = arg(emu, 4) as u64;
    let timeout_str = if timeout_ptr == 0 {
        "infinite".to_string()
    } else {
        let tv_sec = emu.maps.read_dword(timeout_ptr).unwrap_or(0);
        let tv_usec = emu.maps.read_dword(timeout_ptr + 4).unwrap_or(0);
        format!("{}.{:06}s", tv_sec, tv_usec)
    };
    log_red!(
        emu,
        "ws2_32!select nfds:{} read:0x{:x} write:0x{:x} except:0x{:x} timeout:{}",
        nfds,
        readfds,
        writefds,
        exceptfds,
        timeout_str
    );
    // Return 0 = timeout, no sockets ready
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 5);
}
