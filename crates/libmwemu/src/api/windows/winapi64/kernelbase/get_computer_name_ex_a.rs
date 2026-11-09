use crate::emu;

pub fn GetComputerNameExA(emu: &mut emu::Emu) {
    let _name_type = emu.regs().rcx as u32;
    let buf = emu.regs().rdx;
    let size_ptr = emu.regs().r8;
    let name = "DESKTOP";
    log_red!(emu, "kernelbase!GetComputerNameExA buf:0x{:x}", buf);
    if buf != 0 {
        emu.maps.write_string(buf, name);
    }
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, name.len() as u32);
    }
    emu.regs_mut().rax = 1;
}
