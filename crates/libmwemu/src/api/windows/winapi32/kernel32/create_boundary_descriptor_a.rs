use crate::emu;
use crate::windows::constants;

pub fn CreateBoundaryDescriptorA(emu: &mut emu::Emu) {
    let Name = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _Flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");

    let Name = emu.maps.read_string(Name as u64);

    log_red!(emu, "kernel32!CreateBoundaryDescriptorA {}", Name);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
