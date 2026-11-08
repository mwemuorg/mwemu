use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn check_allow_decrypted_remote_destination_policy(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CheckAllowDecryptedRemoteDestinationPolicy");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
