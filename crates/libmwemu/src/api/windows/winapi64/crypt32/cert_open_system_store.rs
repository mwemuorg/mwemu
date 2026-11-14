use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn CertOpenSystemStore(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "crypt32!CertOpenSystemStore arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.regs_mut().rax = helper::handler_create("CertOpenSystemStore");
}

pub fn CertOpenSystemStoreA(emu: &mut emu::Emu) {
    cert_open_system_store(emu, "CertOpenSystemStoreA", false);
}

fn cert_open_system_store(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hprov = emu.regs().rcx;
    let subsystem = emu.regs().rdx;

    let name = strings::read_text(emu, subsystem, wide);

    log_red!(emu, "crypt32!{api} subsystem: {}", name);

    let handle = helper::handler_create(&format!("cert_sys_store://{}", name));
    emu.regs_mut().rax = handle;
}

pub fn CertOpenSystemStoreW(emu: &mut emu::Emu) {
    cert_open_system_store(emu, "CertOpenSystemStoreW", true);
}
