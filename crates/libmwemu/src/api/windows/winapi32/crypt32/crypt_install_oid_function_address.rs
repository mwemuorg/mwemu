use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptInstallOIDFunctionAddress(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptInstallOIDFunctionAddress arg1:0x{:x}",
        arg1
    );
    let handle = helper::handler_create("CryptInstallOIDFunctionAddress");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 6);
}
