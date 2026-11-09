use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CertCloseStore(emu: &mut emu::Emu) {
    let h_store = arg(emu, 0) as u64;
    let flags = arg(emu, 1);

    log_red!(
        emu,
        "crypt32!CertCloseStore handle: 0x{:x} flags: 0x{:x}",
        h_store,
        flags
    );

    helper::handler_close(h_store);

    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
