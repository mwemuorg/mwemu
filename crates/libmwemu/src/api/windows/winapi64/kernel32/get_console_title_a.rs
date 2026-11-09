use crate::emu;

pub fn GetConsoleTitleA(emu: &mut emu::Emu) {
    let buff_ptr = emu.regs().rcx;
    let size = emu.regs().rdx as u32;

    log_red!(emu, "kernel32!GetConsoleTitleA size:{}", size);

    let title = "Windows Console";
    if buff_ptr != 0 && size > 0 {
        emu.maps.write_string(buff_ptr, title);
    }

    emu.regs_mut().rax = title.len() as u64;
}
