use crate::emu;

pub fn SHFileOperation(emu: &mut emu::Emu) {
    let file_op = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let func = if file_op != 0 {
        emu.maps.read_dword(file_op + 4).unwrap_or(0)
    } else {
        0
    };
    let from_ptr = if file_op != 0 {
        emu.maps.read_dword(file_op + 8).unwrap_or(0) as u64
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
        "shell32!SHFileOperation op:{} from:'{}' ptr:0x{:x}",
        func,
        from,
        file_op
    );
    emu.stack_pop32(false);
    emu.regs_mut().rax = 0;
}

pub fn SHFileOperationA(emu: &mut emu::Emu) {
    let file_op = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let func = if file_op != 0 {
        emu.maps.read_dword(file_op + 4).unwrap_or(0)
    } else {
        0
    };
    let from_ptr = if file_op != 0 {
        emu.maps.read_dword(file_op + 8).unwrap_or(0) as u64
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
    emu.stack_pop32(false);
    emu.regs_mut().rax = 0;
}

pub fn SHFileOperationW(emu: &mut emu::Emu) {
    let file_op = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let func = if file_op != 0 {
        emu.maps.read_dword(file_op + 4).unwrap_or(0)
    } else {
        0
    };
    let from_ptr = if file_op != 0 {
        emu.maps.read_dword(file_op + 8).unwrap_or(0) as u64
    } else {
        0
    };
    let from = if from_ptr != 0 {
        emu.maps.read_wide_string(from_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "shell32!SHFileOperationW op:{} from:'{}' ptr:0x{:x}",
        func,
        from,
        file_op
    );
    emu.stack_pop32(false);
    emu.regs_mut().rax = 0;
}
