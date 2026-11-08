use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CertOpenStore(emu: &mut emu::Emu) {
    let store_provider = arg(emu, 0);
    let encoding_type = arg(emu, 1);
    let _hcrypt_prov = arg(emu, 2);
    let flags = arg(emu, 3);

    log_red!(
        emu,
        "crypt32!CertOpenStore provider: 0x{:x} enc: 0x{:x} flags: 0x{:x}",
        store_provider,
        encoding_type,
        flags
    );

    let handle = helper::handler_create("cert_store");
    emu.regs_mut().rax = handle;
    stdcall_ret(emu, 5);
}
