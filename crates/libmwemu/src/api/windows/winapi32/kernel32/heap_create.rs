use crate::emu;
use crate::winapi::helper;

pub fn HeapCreate(emu: &mut emu::Emu) {
    let opts = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!HeapCreate cannot read opts");
    let init_sz = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!HeapCreate cannot read init_sz");
    let max_sz = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!HeapCreate cannot read max_sz");

    log_red!(
        emu,
        "kernel32!HeapCreate initSz: {} maxSz: {}",
        init_sz,
        max_sz
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = helper::handler_create("heap://");
}
