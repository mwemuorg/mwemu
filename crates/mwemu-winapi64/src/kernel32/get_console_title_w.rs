use crate::emu;

pub fn GetConsoleTitleW(emu: &mut emu::Emu) {
    let buff_ptr = emu.regs().rcx;
    let size = emu.regs().rdx as u32;

    log_red!(emu, "kernel32!GetConsoleTitleW size:{}", size);

    let title = "Windows Console";
    if buff_ptr != 0 && size > 0 {
        emu.maps.write_wide_string(buff_ptr, title);
    }

    emu.regs_mut().rax = title.chars().count() as u64;
}
