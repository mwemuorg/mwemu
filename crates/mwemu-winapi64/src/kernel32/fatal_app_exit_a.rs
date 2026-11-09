use crate::emu;

pub fn FatalAppExitA(emu: &mut emu::Emu) {
    let _action = emu.regs().rcx;
    let msg_ptr = emu.regs().rdx;

    let msg = if msg_ptr != 0 {
        emu.maps.read_string(msg_ptr)
    } else {
        String::new()
    };

    log_red!(emu, "kernel32!FatalAppExitA '{}'", msg);

    emu.stop();
}
