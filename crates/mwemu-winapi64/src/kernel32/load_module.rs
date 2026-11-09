use crate::emu;

pub fn LoadModule(emu: &mut emu::Emu) {
    let lp_module_name = emu.regs().rcx;
    let lp_param_block = emu.regs().rdx;

    let name = emu.maps.read_string(lp_module_name);
    log_red!(
        emu,
        "kernel32!LoadModule name:'{}' paramBlock:0x{:x}",
        name,
        lp_param_block
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 32;
}
