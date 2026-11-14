use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CertFindCertificateInStore(emu: &mut emu::Emu) {
    let h_store = arg(emu, 0);
    let _encoding_type = arg(emu, 1);
    let _find_flags = arg(emu, 2);
    let find_type = arg(emu, 3);

    log_red!(
        emu,
        "crypt32!CertFindCertificateInStore store: 0x{:x} find_type: 0x{:x}",
        h_store,
        find_type
    );

    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 6);
}
