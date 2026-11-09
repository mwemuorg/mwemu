use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn WinHttpReadData(emu: &mut emu::Emu) {
    let request = arg(emu, 0);
    let bytes_read_ptr = arg(emu, 3) as u64;
    log_red!(emu, "winhttp!WinHttpReadData request: 0x{:x}", request);
    if bytes_read_ptr != 0 {
        emu.maps.write_dword(bytes_read_ptr, 0); // orderly end of data
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
