use crate::emu;

pub fn ShellExecuteA(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let lp_operation = emu.regs().rdx;
    let lp_file = emu.regs().r8;
    let lp_parameters = emu.regs().r9;
    let lp_directory = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let n_show_cmd = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    let operation = if lp_operation != 0 {
        emu.maps.read_string(lp_operation)
    } else {
        "open".to_string()
    };
    let file = emu.maps.read_string(lp_file);
    let params = if lp_parameters != 0 {
        emu.maps.read_string(lp_parameters)
    } else {
        "".to_string()
    };

    log_red!(
        emu,
        "shell32!ShellExecuteA op: {} file: {} params: {}",
        operation,
        file,
        params
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 42; // > 32 means success
}
