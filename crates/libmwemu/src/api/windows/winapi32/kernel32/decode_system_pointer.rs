use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn DecodeSystemPointer(emu: &mut emu::Emu) {
    let ptr = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!DecodeSystemPointer ptr:0x{:x}", ptr);
    let src = arg(emu, 0) as u64;
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = emu.maps.read_string(src);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = ptr;
    stdcall_ret(emu, 1);
}
