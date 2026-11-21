use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GlobalMemoryStatusEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let buf_ptr = api_try!(emu, call.arg(emu, 0));
    if buf_ptr != 0 {
        emu.maps.write_dword(buf_ptr, 64); // dwLength
        emu.maps.write_dword(buf_ptr + 4, 50); // dwMemoryLoad (50%)
        emu.maps.write_qword(buf_ptr + 8, 8 * 1024 * 1024 * 1024); // ullTotalPhys (8GB)
        emu.maps.write_qword(buf_ptr + 16, 4 * 1024 * 1024 * 1024); // ullAvailPhys (4GB)
        emu.maps.write_qword(buf_ptr + 24, 16 * 1024 * 1024 * 1024); // ullTotalPageFile
        emu.maps.write_qword(buf_ptr + 32, 8 * 1024 * 1024 * 1024); // ullAvailPageFile
        emu.maps
            .write_qword(buf_ptr + 40, 128u64 * 1024 * 1024 * 1024); // ullTotalVirtual
        emu.maps
            .write_qword(buf_ptr + 48, 128u64 * 1024 * 1024 * 1024); // ullAvailVirtual
        emu.maps.write_qword(buf_ptr + 56, 0); // ullAvailExtendedVirtual
    }
    log_red!(emu, "kernelbase!GlobalMemoryStatusEx ptr:0x{:x}", buf_ptr);
    call.set_return(emu, 1);
}
