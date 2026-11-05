use crate::emu;

pub fn GetNamedPipeAttribute(emu: &mut emu::Emu) {
    let h_pipe = emu.regs().rcx;
    let attr_type = emu.regs().rdx as u32;
    let attr_name_ptr = emu.regs().r8;
    let attr_value_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetNamedPipeAttribute h:0x{:x} type:{} name:0x{:x} val:0x{:x}",
        h_pipe,
        attr_type,
        attr_name_ptr,
        attr_value_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
