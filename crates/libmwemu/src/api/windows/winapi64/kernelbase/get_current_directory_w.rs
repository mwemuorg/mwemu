use crate::emu;

pub fn GetCurrentDirectoryW(emu: &mut emu::Emu) {
    let size = emu.regs().rcx as u32;
    let buf = emu.regs().rdx;
    let cwd = "C:\\Users\\admin";
    log_red!(
        emu,
        "kernelbase!GetCurrentDirectoryW buf:0x{:x} sz:{}",
        buf,
        size
    );
    if buf != 0 && (cwd.len() as u32) < size {
        for (i, ch) in cwd.encode_utf16().enumerate() {
            emu.maps.write_word(buf + (i as u64) * 2, ch);
        }
        emu.maps.write_word(buf + (cwd.len() as u64) * 2, 0);
    }
    emu.regs_mut().rax = cwd.len() as u64;
}
