use crate::emu;
use crate::windows::constants;

pub fn GetNamedPipeClientComputerNameA(emu: &mut emu::Emu) {
    let _Pipe = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let ClientComputerName = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let ClientComputerNameLength = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");

    let ClientComputerName = emu.maps.read_string(ClientComputerName as u64);
    let ClientComputerNameLength = emu.maps.read_string(ClientComputerNameLength as u64);

    log_red!(
        emu,
        "kernel32!GetNamedPipeClientComputerNameA {} {}",
        ClientComputerName,
        ClientComputerNameLength
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
