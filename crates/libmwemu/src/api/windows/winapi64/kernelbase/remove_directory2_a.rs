use crate::emu;

pub fn RemoveDirectory2A(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let path = emu.maps.read_string(path_ptr);
    log_red!(emu, "kernelbase!RemoveDirectory2A '{}'", path);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
