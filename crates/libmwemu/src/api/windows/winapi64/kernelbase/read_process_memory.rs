use crate::emu;

pub fn ReadProcessMemory(emu: &mut emu::Emu) {
    let _process = emu.regs().rcx;
    let base_addr = emu.regs().rdx;
    let buf_ptr = emu.regs().r8;
    let size = emu.regs().r9 as usize;
    let bytes_read_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    if buf_ptr != 0 && base_addr != 0 && size > 0 {
        for i in 0..size {
            let b = emu.maps.read_byte(base_addr + i as u64).unwrap_or(0);
            emu.maps.write_byte(buf_ptr + i as u64, b);
        }
    }
    if bytes_read_ptr != 0 {
        emu.maps.write_qword(bytes_read_ptr, size as u64);
    }
    log_red!(
        emu,
        "kernelbase!ReadProcessMemory addr:0x{:x} buf:0x{:x} sz:{}",
        base_addr,
        buf_ptr,
        size
    );
    emu.regs_mut().rax = 1;
}
