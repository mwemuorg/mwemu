use crate::emu;

pub fn InternetCheckConnectionW(emu: &mut emu::Emu) {
    let url_ptr = emu.regs().rcx;
    let flags = emu.regs().rdx as u32;
    let url = if url_ptr != 0 {
        emu.maps.read_wide_string(url_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!InternetCheckConnectionW url:'{}' flags:0x{:x}",
        url,
        flags
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
