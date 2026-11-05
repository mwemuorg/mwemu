use crate::emu;
use crate::windows::constants;

pub fn DnsHostnameToComputerNameA(emu: &mut emu::Emu) {
    let Hostname = emu.regs().rcx as u32;
    let ComputerName = emu.regs().rdx as u32;
    let _nSize = emu.regs().r8 as u32;

    let Hostname = emu.maps.read_string(Hostname as u64);
    let ComputerName = emu.maps.read_string(ComputerName as u64);

    log_red!(
        emu,
        "kernel32!DnsHostnameToComputerNameA {} {}",
        Hostname,
        ComputerName
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
