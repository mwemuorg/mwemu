use crate::emu;
use crate::winapi::helper;

pub fn LockResource(emu: &mut emu::Emu) {
    let hResData = emu.regs().rcx;

    emu.stack_pop32(false);

    if helper::handler_exist(hResData) {
        let uri = helper::handler_get_uri(hResData);
        let ptr = uri.split("_").next().unwrap().parse::<u64>().unwrap() + emu.base;

        log_red!(
            emu,
            "** {} kernel32!LockResource {:x} {:x}",
            emu.pos,
            hResData,
            ptr
        );
        emu.regs_mut().rax = ptr;
        return;
    }

    log_red!(
        emu,
        "** {} kernel32!LockResource {:x} not found",
        emu.pos,
        hResData
    );

    emu.regs_mut().rax = 0;
}
