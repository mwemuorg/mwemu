use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CertGetServerOcspResponseContext(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    stdcall_ret(emu, 3);
    log_red!(
        emu,
        "crypt32!CertGetServerOcspResponseContext arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.regs_mut().rax = helper::handler_create("CertGetServerOcspResponseContext");
}
