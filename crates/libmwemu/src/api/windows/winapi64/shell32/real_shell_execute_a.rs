use crate::emu;

pub fn RealShellExecuteA(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let operation = emu.regs().rdx;
    let file_ptr = emu.regs().r8;
    let params_ptr = emu.regs().r9;
    let dir = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let bShowWindow = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    let file = emu.maps.read_string(file_ptr);
    let params = emu.maps.read_string(params_ptr);

    log_red!(emu, "shell32!RealShellExecuteA {} {}", file, params);

    emu.set_last_error(0);
    emu.regs_mut().rax = 34;
}
