use crate::emu;

pub fn Toolhelp32ReadProcessMemory(emu: &mut emu::Emu) {
    let process = emu.regs().rcx;
    let base_addr = emu.regs().rdx;
    let buffer = emu.regs().r8;
    let size = emu.regs().r9 as usize;

    log_red!(
        emu,
        "kernel32!Toolhelp32ReadProcessMemory proc:0x{:x} addr:0x{:x} size:{}",
        process,
        base_addr,
        size
    );

    if buffer != 0 && base_addr != 0 && size > 0 && size <= 0x100000 {
        let data = emu.maps.read_bytes(base_addr, size).to_vec();
        emu.maps.write_bytes(buffer, &data);
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
