use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetNamedPipeAttribute(emu: &mut emu::Emu) {
    let h_pipe = arg(emu, 0) as u64;
    let attr_type = arg(emu, 1);
    let attr_name_ptr = arg(emu, 2) as u64;
    let attr_value_ptr = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!GetNamedPipeAttribute h:0x{:x} type:{} name:0x{:x} val:0x{:x}",
        h_pipe,
        attr_type,
        attr_name_ptr,
        attr_value_ptr
    );
    let out_ptr = arg(emu, 3) as u64;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}
