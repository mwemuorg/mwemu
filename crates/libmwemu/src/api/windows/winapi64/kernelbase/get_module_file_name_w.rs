use crate::emu;

pub fn GetModuleFileNameW(emu: &mut emu::Emu) {
    let module = emu.regs().rcx;
    let buf_ptr = emu.regs().rdx;
    let buf_size = emu.regs().r8;
    let name = "C:\\mwemu\\program.exe";
    if buf_ptr != 0 && buf_size > 0 {
        emu.maps.write_wide_string(buf_ptr, name);
    }
    log_red!(
        emu,
        "kernelbase!GetModuleFileNameW module:0x{:x} => '{}'",
        module,
        name
    );
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = name.len() as u64;
}
