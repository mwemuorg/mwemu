use crate::emu;
use crate::windows::constants;

pub fn RemoveLocalAlternateComputerNameW(emu: &mut emu::Emu) {
    let Hostname = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _a2 = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");

    let Hostname = emu.maps.read_wide_string(Hostname as u64);

    log_red!(
        emu,
        "kernel32!RemoveLocalAlternateComputerNameW {}",
        Hostname
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
