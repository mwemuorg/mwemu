use crate::emu;
use crate::winapi::helper;

pub fn CertOpenSystemStoreW(emu: &mut emu::Emu) {
    let hprov = emu.regs().rcx;
    let subsystem = emu.regs().rdx;

    let name = emu.maps.read_wide_string(subsystem);

    log_red!(emu, "crypt32!CertOpenSystemStoreW subsystem: {}", name);

    let handle = helper::handler_create(&format!("cert_sys_store://{}", name));
    emu.regs_mut().rax = handle;
}
