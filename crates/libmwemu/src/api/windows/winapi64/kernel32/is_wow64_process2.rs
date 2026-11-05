use crate::emu;

pub fn IsWow64Process2(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    let p_process_machine = emu.regs().rdx;
    let p_native_machine = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!IsWow64Process2 hProcess:0x{:x} pProcessMachine:0x{:x} pNativeMachine:0x{:x}",
        h_process,
        p_process_machine,
        p_native_machine
    );
    if p_process_machine != 0 {
        emu.maps.write_dword(p_process_machine, 0);
    }
    if p_native_machine != 0 {
        emu.maps.write_dword(p_native_machine, 0x8664);
    }
    emu.regs_mut().rax = 1;
}
