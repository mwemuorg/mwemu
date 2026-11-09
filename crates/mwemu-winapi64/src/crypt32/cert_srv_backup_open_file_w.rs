use crate::emu;
use crate::winapi::helper;

pub fn CertSrvBackupOpenFileW(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let s = emu.maps.read_wide_string(str_ptr);
    let hndl = helper::handler_create("CertSrvBackupOpenFileW");
    log_red!(
        emu,
        "crypt32!CertSrvBackupOpenFileW '{}' => 0x{:x}",
        s,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
