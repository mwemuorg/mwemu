use crate::emu;

pub fn PathIsContentTypeW(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let content_type_ptr = emu.regs().rdx;

    let mut path = String::new();
    let mut content_type = String::new();

    if path_ptr > 0 {
        path = emu.maps.read_wide_string(path_ptr);
    }
    if content_type_ptr > 0 {
        content_type = emu.maps.read_wide_string(content_type_ptr);
    }

    log_red!(
        emu,
        "shlwapi!PathIsContentTypeW path: {} content-type: {}",
        path,
        content_type
    );

    emu.regs_mut().rax = 1;
}
