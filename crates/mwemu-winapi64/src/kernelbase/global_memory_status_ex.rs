use crate::emu;

pub fn GlobalMemoryStatusEx(emu: &mut emu::Emu) {
    let buf_ptr = emu.regs().rcx;
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
    emu.regs_mut().rax = 1;
}
