use crate::emu;
use crate::windows::constants;

pub fn GetNamedPipeClientComputerNameA(emu: &mut emu::Emu) {
    let _Pipe = emu.regs().rcx as u32;
    let ClientComputerName = emu.regs().rdx as u32;
    let ClientComputerNameLength = emu.regs().r8 as u32;

    let ClientComputerName = emu.maps.read_string(ClientComputerName as u64);
    let ClientComputerNameLength = emu.maps.read_string(ClientComputerNameLength as u64);

    log_red!(
        emu,
        "kernel32!GetNamedPipeClientComputerNameA {} {}",
        ClientComputerName,
        ClientComputerNameLength
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
