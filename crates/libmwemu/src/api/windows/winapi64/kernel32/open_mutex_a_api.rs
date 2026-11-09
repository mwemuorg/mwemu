use crate::emu;

pub fn OpenMutexA_api(emu: &mut emu::Emu) {
    let access = emu.regs().rcx;
    let inherit = emu.regs().rdx;
    let name = emu.regs().r8;
    let name_s = if name != 0 {
        emu.maps.read_string(name)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernel32!OpenMutexA access:0x{:x} name:'{}' => 0",
        access,
        name_s
    );
    emu.regs_mut().rax = 0;
    emu.set_last_error(2); // ERROR_FILE_NOT_FOUND
}
