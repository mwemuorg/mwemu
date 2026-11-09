use crate::emu;

pub fn GetCurrentDirectoryA(emu: &mut emu::Emu) {
    let size = emu.regs().rcx as u32;
    let buf = emu.regs().rdx;
    let cwd = "C:\\Users\\admin";
    log_red!(
        emu,
        "kernelbase!GetCurrentDirectoryA buf:0x{:x} sz:{}",
        buf,
        size
    );
    if buf != 0 && (cwd.len() as u32) < size {
        emu.maps.write_string(buf, cwd);
    }
    emu.regs_mut().rax = cwd.len() as u64;
}
