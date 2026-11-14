use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CertOpenStore(emu: &mut emu::Emu) {
    let store_provider = emu.regs().rcx;
    let encoding_type = emu.regs().rdx;
    let hcrypt_prov = emu.regs().r8;
    let flags = emu.regs().r9;
    let pv_para = WinApiAbi::arg(emu, 4);

    log_red!(
        emu,
        "crypt32!CertOpenStore provider: 0x{:x} enc: 0x{:x} flags: 0x{:x}",
        store_provider,
        encoding_type,
        flags
    );

    let handle = helper::handler_create("cert_store");
    emu.regs_mut().rax = handle;
}
