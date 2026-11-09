use crate::emu;

pub fn WaitNamedPipeW(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().rcx;
    let timeout = emu.regs().rdx;

    let name = if name_ptr != 0 {
        emu.maps.read_wide_string(name_ptr)
    } else {
        String::new()
    };

    log_red!(
        emu,
        "kernel32!WaitNamedPipeW '{}' timeout:{}",
        name,
        timeout
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
