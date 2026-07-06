use crate::emu;

pub fn ReadProcessMemory(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let addr = emu.regs().rdx;
    let buff = emu.regs().r8;
    let size = emu.regs().r9;
    let bytes = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("kernel32!ReadProcessMemory cannot read bytes");

    log_red!(
        emu,
        "kernel32!ReadProcessMemory hndl: {} from: 0x{:x} to: 0x{:x} sz: {}",
        hndl,
        addr,
        buff,
        size
    );

    emu.maps.write_qword(bytes, size);
    emu.maps.memset(buff, 0x90, size as usize);

    emu.regs_mut().rax = 1;
}
