use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CertAddEncodedCTLToStore(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    let arg5 = WinApiAbi::arg(emu, 4);
    let arg6 = WinApiAbi::arg(emu, 5);
    log_red!(
        emu,
        "crypt32!CertAddEncodedCTLToStore 0x{:x} 0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create("CertAddEncodedCTLToStore");
    let out_ptr = WinApiAbi::arg(emu, 5);
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
