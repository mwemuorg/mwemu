use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn QueryServiceStatusEx(emu: &mut emu::Emu) {
    let hsvc = emu.regs().rcx;
    let info_level = emu.regs().rdx;
    let buf_ptr = emu.regs().r8;
    let buf_size = emu.regs().r9;
    let bytes_needed_ptr = WinApiAbi::arg(emu, 4);
    // SERVICE_STATUS_PROCESS is 36 bytes
    if buf_ptr != 0 && buf_size >= 36 {
        emu.maps.write_dword(buf_ptr, 0x10); // dwServiceType = SERVICE_WIN32_OWN_PROCESS
        emu.maps.write_dword(buf_ptr + 4, 4); // dwCurrentState = SERVICE_RUNNING
        emu.maps.write_dword(buf_ptr + 8, 0x1f); // dwControlsAccepted
        emu.maps.write_dword(buf_ptr + 12, 0);
        emu.maps.write_dword(buf_ptr + 16, 0);
        emu.maps.write_dword(buf_ptr + 20, 0);
        emu.maps.write_dword(buf_ptr + 24, 0);
        emu.maps.write_dword(buf_ptr + 28, 1234); // dwProcessId
        emu.maps.write_dword(buf_ptr + 32, 0); // dwServiceFlags
    }
    if bytes_needed_ptr != 0 {
        emu.maps.write_dword(bytes_needed_ptr, 36);
    }
    log_red!(
        emu,
        "advapi32!QueryServiceStatusEx svc:0x{:x} level:{}",
        hsvc,
        info_level
    );
    emu.regs_mut().rax = 1;
}
