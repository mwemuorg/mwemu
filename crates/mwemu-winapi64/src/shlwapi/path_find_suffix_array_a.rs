use crate::emu;

pub fn PathFindSuffixArrayA(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let suffixes_ptr = emu.regs().rdx;

    let mut path = String::new();
    let mut suffixes = String::new();

    if path_ptr > 0 {
        path = emu.maps.read_string(path_ptr);
    }
    if suffixes_ptr > 0 {
        suffixes = emu.maps.read_string(suffixes_ptr);
    }

    log_red!(
        emu,
        "shlwapi!PathFindSuffixArrayA path: {} suffixes: {}",
        path,
        suffixes
    );

    emu.regs_mut().rax = emu.regs().rdx;
}
