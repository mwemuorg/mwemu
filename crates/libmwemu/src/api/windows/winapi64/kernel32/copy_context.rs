use crate::emu;

pub fn CopyContext(emu: &mut emu::Emu) {
    let dest = emu.regs().rcx;
    let context_flags = emu.regs().rdx;
    let src = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!CopyContext dest:0x{:x} flags:0x{:x} src:0x{:x}",
        dest,
        context_flags,
        src
    );

    if dest != 0 && src != 0 {
        let size = 1232usize;
        let data = emu.maps.read_bytes(src, size).to_vec();
        emu.maps.write_bytes(dest, &data);
    }
    emu.regs_mut().rax = 1;
}
