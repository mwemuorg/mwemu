use crate::emu;

pub fn VirtualQuery(emu: &mut emu::Emu, addr: u64) {
    let addr = emu.regs().rcx;
    let buf_ptr = emu.regs().rdx;
    let buf_len = emu.regs().r8;
    if buf_ptr != 0 && buf_len >= 48 {
        emu.maps.write_qword(buf_ptr, addr); // BaseAddress
        emu.maps.write_qword(buf_ptr + 8, addr); // AllocationBase
        emu.maps.write_dword(buf_ptr + 16, 0x40); // AllocationProtect = PAGE_EXECUTE_READWRITE
        emu.maps.write_qword(buf_ptr + 24, 0x1000); // RegionSize
        emu.maps.write_dword(buf_ptr + 32, 0x1000); // State = MEM_COMMIT
        emu.maps.write_dword(buf_ptr + 36, 0x40); // Protect = PAGE_EXECUTE_READWRITE
        emu.maps.write_dword(buf_ptr + 40, 0x20000); // Type = MEM_PRIVATE
    }
    log_red!(emu, "kernelbase!VirtualQuery addr:0x{:x} => 48", addr);
    emu.set_last_error(0);
    emu.regs_mut().rax = 48;
}
