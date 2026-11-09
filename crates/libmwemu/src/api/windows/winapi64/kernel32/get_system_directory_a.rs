use crate::emu;

pub fn GetSystemDirectoryA(emu: &mut emu::Emu) {
    let out_buff_ptr = emu.regs().rcx;
    let size = emu.regs().rdx;

    let output = emu.cfg.system_directory.clone();
    emu.maps.write_string(out_buff_ptr, &output);

    log_red!(emu, "kernel32!GetSystemDirectoryA");

    emu.regs_mut().rax = output.len() as u64;
}
