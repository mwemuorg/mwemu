use crate::emu;
use crate::windows::constants;

pub fn SetLocalPrimaryComputerNameW(emu: &mut emu::Emu) {
    let String = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _a2 = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");

    let String = emu.maps.read_wide_string(String as u64);

    log_red!(emu, "kernel32!SetLocalPrimaryComputerNameW {}", String);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
