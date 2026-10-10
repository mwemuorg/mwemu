use crate::emu;

pub fn DuplicateHandle(emu: &mut emu::Emu) {
    let _src_process = emu.regs().rcx;
    let src_handle = emu.regs().rdx;
    let _target_process = emu.regs().r8;
    let target_handle_ptr = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!DuplicateHandle src: 0x{:x} target_ptr: 0x{:x}",
        src_handle,
        target_handle_ptr
    );

    emu.maps.write_qword(target_handle_ptr, src_handle);
    emu.regs_mut().rax = 1;
}
