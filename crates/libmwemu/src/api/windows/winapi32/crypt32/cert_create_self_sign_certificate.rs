use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CertCreateSelfSignCertificate(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
    let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
    let arg5 = emu.maps.read_dword(esp + 20).unwrap_or(0) as u64;
    let arg6 = emu.maps.read_dword(esp + 24).unwrap_or(0) as u64;
    stdcall_ret(emu, 8);
    log_red!(
        emu,
        "crypt32!CertCreateSelfSignCertificate arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x} arg6:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4,
        arg5,
        arg6
    );
    emu.regs_mut().rax = helper::handler_create("CertCreateSelfSignCertificate");
}
