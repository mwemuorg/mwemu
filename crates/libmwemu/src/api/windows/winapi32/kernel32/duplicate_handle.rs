use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn DuplicateHandle(emu: &mut emu::Emu) {
    let src_handle = arg(emu, 1);
    let target_handle_ptr = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!DuplicateHandle src: 0x{:x} target_ptr: 0x{:x}",
        src_handle,
        target_handle_ptr
    );
    emu.maps.write_dword(target_handle_ptr, src_handle);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 7);
}
