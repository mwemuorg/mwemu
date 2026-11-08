use crate::emu;

pub(super) fn PathFileExistsA(emu: &mut emu::Emu) {
    let ptr_path = emu.regs().rcx;
    let path = emu.maps.read_string(ptr_path);

    log_red!(emu, "shlwapi!PathFileExistsA path: {}", path);

    emu.regs_mut().rax = 1; // True
}
