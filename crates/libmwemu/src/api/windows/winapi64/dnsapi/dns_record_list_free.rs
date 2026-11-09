use crate::emu;

pub fn DnsRecordListFree(emu: &mut emu::Emu) {
    let record = emu.regs().rcx;
    log_red!(emu, "dnsapi!DnsRecordListFree ptr:0x{:x}", record);
}
