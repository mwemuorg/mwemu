use crate::emu;

pub fn GetWindowsDirectoryW(emu: &mut emu::Emu) {
    let lp_buffer = emu.regs().rcx;
    let u_size = emu.regs().rdx as usize;
    log_red!(
        emu,
        "** {} kernel32!GetWindowsDirectoryW lp_buffer: 0x{:x} u_size: {}",
        emu.pos,
        lp_buffer,
        u_size
    );
    let output = emu.cfg.windows_directory.clone();
    if emu.maps.is_mapped(lp_buffer) && u_size > output.len() * 2 + 2 {
        emu.maps.write_wide_string(lp_buffer, &output);
        emu.regs_mut().rax = output.len() as u64 * 2;
    } else {
        emu.regs_mut().rax = 0;
    }
}
