use crate::emu;

pub fn SHFileOperationA(emu: &mut emu::Emu) {
    let file_op = emu.regs().rcx;
    let func = if file_op != 0 {
        emu.maps.read_dword(file_op + 8).unwrap_or(0)
    } else {
        0
    };
    let from_ptr = if file_op != 0 {
        emu.maps.read_qword(file_op + 16).unwrap_or(0)
    } else {
        0
    };
    let from = if from_ptr != 0 {
        emu.maps.read_string(from_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "shell32!SHFileOperationA op:{} from:'{}' ptr:0x{:x}",
        func,
        from,
        file_op
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
