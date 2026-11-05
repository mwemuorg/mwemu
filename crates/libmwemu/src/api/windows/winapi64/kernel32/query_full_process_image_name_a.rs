use crate::emu;

pub fn QueryFullProcessImageNameA(emu: &mut emu::Emu) {
    let _h_process = emu.regs().rcx;
    let _flags = emu.regs().rdx;
    let name_ptr = emu.regs().r8;
    let size_ptr = emu.regs().r9;

    let name = "C:\\Windows\\System32\\emulated.exe";

    if name_ptr != 0 {
        emu.maps.write_string(name_ptr, name);
    }
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, name.len() as u32);
    }

    log_red!(
        emu,
        "kernel32!QueryFullProcessImageNameA => '{}'",
        name
    );
    emu.regs_mut().rax = 1;
}
