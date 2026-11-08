use crate::emu;

pub(super) fn PathFileExistsA(emu: &mut emu::Emu) {
    let ptr_path = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    let path = emu.maps.read_string(ptr_path);

    log_red!(emu, "shlwapi!PathFileExistsA path: {}", path);

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1; // True
}
