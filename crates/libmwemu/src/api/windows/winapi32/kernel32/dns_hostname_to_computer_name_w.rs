use crate::emu;
use crate::windows::constants;

pub fn DnsHostnameToComputerNameW(emu: &mut emu::Emu) {
    let Hostname = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let ComputerName = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _nSize = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");

    let Hostname = emu.maps.read_wide_string(Hostname as u64);
    let ComputerName = emu.maps.read_wide_string(ComputerName as u64);

    log_red!(
        emu,
        "kernel32!DnsHostnameToComputerNameW {} {}",
        Hostname,
        ComputerName
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
