use super::allocate_memory;
use crate::emu;

pub fn strerror(emu: &mut emu::Emu) {
    let errnum = emu.regs().rcx as u32;
    let msg = match errnum {
        0 => "No error",
        2 => "No such file or directory",
        13 => "Permission denied",
        22 => "Invalid argument",
        _ => "Unknown error",
    };
    let addr = allocate_memory(emu, msg.len() as u64 + 1).unwrap_or(0);
    if addr != 0 {
        emu.maps.write_string(addr, msg);
    }
    log_red!(emu, "msvcrt!strerror {} => '{}'", errnum, msg);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = addr;
}
