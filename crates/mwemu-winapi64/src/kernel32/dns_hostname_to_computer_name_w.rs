
use crate::windows::constants::errors::*;
use crate::emu;

pub fn DnsHostnameToComputerNameW(emu: &mut emu::Emu) {
    let Hostname = emu.regs().rcx as u32;
    let ComputerName = emu.regs().rdx as u32;
    let _nSize = emu.regs().r8 as u32;

    let Hostname = emu.maps.read_wide_string(Hostname as u64);
    let ComputerName = emu.maps.read_wide_string(ComputerName as u64);

    log_red!(
        emu,
        "kernel32!DnsHostnameToComputerNameW {} {}",
        Hostname,
        ComputerName
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
