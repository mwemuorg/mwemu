use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn LoadModule(emu: &mut emu::Emu) {
    let lp_module_name = arg(emu, 0) as u64;
    let lp_param_block = arg(emu, 1) as u64;

    let name = emu.maps.read_string(lp_module_name);
    log_red!(
        emu,
        "kernel32!LoadModule name:'{}' paramBlock:0x{:x}",
        name,
        lp_param_block
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 32;
    stdcall_ret(emu, 2);
}
