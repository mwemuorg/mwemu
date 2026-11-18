use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn QueryServiceStatusEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hsvc = api_try!(emu, call.arg(emu, 0));
    let info_level = api_try!(emu, call.arg(emu, 1));
    let buf_ptr = api_try!(emu, call.arg(emu, 2));
    let buf_size = api_try!(emu, call.arg(emu, 3));
    let bytes_needed_ptr = api_try!(emu, call.arg(emu, 4));
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
    call.set_return(emu, 1);
}
