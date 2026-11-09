use crate::emu;

pub fn GetWindowsDirectoryA(emu: &mut emu::Emu) {
    let lp_buffer = emu.regs().rcx as usize;
    let u_size = emu.regs().rdx as usize;
    log_red!(
        emu,
        "** {} kernel32!GetWindowsDirectoryA lp_buffer: 0x{:x} u_size: {}",
        emu.pos,
        lp_buffer,
        u_size
    );
    let output = emu.cfg.windows_directory.clone();
    emu.maps.write_string(lp_buffer as u64, &output);
    emu.regs_mut().rax = output.len() as u64;
}
