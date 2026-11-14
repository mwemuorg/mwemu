use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn SHFileOperation(emu: &mut emu::Emu) {
    let file_op = WinApiAbi::arg(emu, 0);
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
    sh_file_operation(emu, "SHFileOperationA", false);
}

fn sh_file_operation(emu: &mut emu::Emu, api: &str, wide: bool) {
    let file_op = WinApiAbi::arg(emu, 0);
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
        strings::read_text(emu, from_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "shell32!{api} op:{} from:'{}' ptr:0x{:x}",
        func,
        from,
        file_op
    );
    emu.stack_pop32(false);
    emu.regs_mut().rax = 0;
}

pub fn SHFileOperationW(emu: &mut emu::Emu) {
    sh_file_operation(emu, "SHFileOperationW", true);
}
