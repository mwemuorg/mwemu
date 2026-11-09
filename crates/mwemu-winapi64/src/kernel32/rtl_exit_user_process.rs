use crate::emu;

pub fn RtlExitUserProcess(emu: &mut emu::Emu) {
    let exit_code = emu.regs().rcx;
    log_red!(emu, "ntdll!RtlExitUserProcess code:{}", exit_code);
    emu.stop();
}
