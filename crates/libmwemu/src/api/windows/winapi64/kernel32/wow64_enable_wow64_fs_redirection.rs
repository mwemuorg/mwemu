use crate::emu;

pub fn Wow64EnableWow64FsRedirection(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;

    log_red!(
        emu,
        "kernel32!Wow64EnableWow64FsRedirection param1:0x{:x}",
        param1
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
}
