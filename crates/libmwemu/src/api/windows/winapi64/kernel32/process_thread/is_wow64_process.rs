use crate::emu;

pub fn IsWow64Process(emu: &mut emu::Emu) {
    let _process = emu.regs().rcx;
    let out_ptr = emu.regs().rdx;

    log_red!(emu, "kernel32!IsWow64Process out: 0x{:x}", out_ptr);

    emu.maps.write_qword(out_ptr, 0); // FALSE — 64-bit process
    emu.regs_mut().rax = 1;
}
