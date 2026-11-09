use crate::emu;

pub fn CopySid(emu: &mut emu::Emu) {
    let dest_len = emu.regs().rcx as usize;
    let dest_sid = emu.regs().rdx;
    let src_sid = emu.regs().r8;
    log_red!(
        emu,
        "advapi32!CopySid dest:0x{:x} src:0x{:x} len:{}",
        dest_sid,
        src_sid,
        dest_len
    );
    if dest_sid != 0 && src_sid != 0 && dest_len > 0 {
        let copy_len = dest_len.min(68);
        let data = emu.maps.read_bytes(src_sid, copy_len).to_vec();
        emu.maps.write_bytes(dest_sid, &data);
    }
    emu.regs_mut().rax = 1;
}
