use crate::emu;
use crate::windows::constants;

pub fn ReplacePartitionUnit(emu: &mut emu::Emu) {
    let _TargetPartition = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _SparePartition = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _Flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");

    log_red!(emu, "kernel32!ReplacePartitionUnit");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
