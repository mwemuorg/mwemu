use crate::emu;

pub(super) fn PathFileExistsW(emu: &mut emu::Emu) {
    let ptr_path = emu.regs().rcx;
    let path = emu.maps.read_wide_string(ptr_path);

    log_red!(emu, "shlwapi!PathFileExistsW path: {}", path);

    emu.regs_mut().rax = 1; // True
}
