use crate::emu;

pub fn RtlMoveMemory(emu: &mut emu::Emu) {
    let dest = emu.regs().rcx;
    let src = emu.regs().rdx;
    let len = emu.regs().r8 as usize;

    log_red!(
        emu,
        "kernel32!RtlMoveMemory dest:0x{:x} src:0x{:x} len:{}",
        dest,
        src,
        len
    );

    if dest != 0 && src != 0 && len > 0 && len <= 0x100_0000 {
        let data = emu.maps.read_bytes(src, len).to_vec();
        emu.maps.write_bytes(dest, &data);
    }
}
