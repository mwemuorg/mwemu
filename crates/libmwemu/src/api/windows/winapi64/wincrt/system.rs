use crate::emu;

pub fn system(emu: &mut emu::Emu, api: &str) {
    let cmd_ptr = emu.regs().rcx;
    let cmd = if api == "_wsystem" {
        emu.maps.read_wide_string(cmd_ptr)
    } else {
        emu.maps.read_string(cmd_ptr)
    };
    log_red!(emu, "wincrt!{} cmd:'{}'", api, cmd);
    emu.regs_mut().rax = 0;
}
