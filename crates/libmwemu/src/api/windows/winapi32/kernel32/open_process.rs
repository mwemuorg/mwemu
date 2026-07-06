use crate::emu;
use crate::winapi::helper;

pub fn OpenProcess(emu: &mut emu::Emu) {
    let access = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!OpenProcess cannot read access");
    let inherit = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!OpenProcess cannot read inherit");
    let pid = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!OpenProcess cannot read pid");

    log_red!(emu, "kernel32!OpenProcess pid: {}", pid);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    let uri = format!("pid://{}", pid);
    emu.regs_mut().rax = helper::handler_create(&uri);
}
