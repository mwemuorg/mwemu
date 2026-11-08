use crate::emu;

pub fn GetComputerNameExW(emu: &mut emu::Emu) {
    let _name_type = emu.regs().rcx as u32;
    let buf = emu.regs().rdx;
    let size_ptr = emu.regs().r8;
    let name = "DESKTOP";
    log_red!(emu, "kernelbase!GetComputerNameExW buf:0x{:x}", buf);
    if buf != 0 {
        for (i, ch) in name.encode_utf16().enumerate() {
            emu.maps.write_word(buf + (i as u64) * 2, ch);
        }
        emu.maps.write_word(buf + (name.len() as u64) * 2, 0);
    }
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, name.len() as u32);
    }
    emu.regs_mut().rax = 1;
}
