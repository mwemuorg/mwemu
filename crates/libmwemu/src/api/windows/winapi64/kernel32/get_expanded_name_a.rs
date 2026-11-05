use crate::emu;

pub fn GetExpandedNameA(emu: &mut emu::Emu) {
    let source_ptr = emu.regs().rcx;
    let dest_ptr = emu.regs().rdx;
    let source = emu.maps.read_string(source_ptr);
    log_red!(emu, "kernel32!GetExpandedNameA '{}'", source);
    if dest_ptr != 0 && !source.is_empty() {
        emu.maps.write_string(dest_ptr, &source);
    }
    emu.regs_mut().rax = 1;
}
