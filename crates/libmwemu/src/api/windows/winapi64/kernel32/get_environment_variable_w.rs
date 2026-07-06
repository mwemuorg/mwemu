use crate::emu;

pub fn GetEnvironmentVariableW(emu: &mut emu::Emu) {
    let lp_name = emu.regs().rcx as usize;
    let lp_buffer = emu.regs().rdx as usize;
    let n_size = emu.regs().r8 as usize;
    let name = emu.maps.read_wide_string(lp_name as u64);
    log_red!(
        emu,
        "** {} kernel32!GetEnvironmentVariableW {:x} {:x} {:x} name: {}",
        emu.pos,
        lp_name,
        lp_buffer,
        n_size,
        name
    );
    // TODO: implement this
    emu.regs_mut().rax = 1;
}
