use crate::emu;

pub fn CheckAllowDecryptedRemoteDestinationPolicy(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "kernel32!CheckAllowDecryptedRemoteDestinationPolicy => S_OK"
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
