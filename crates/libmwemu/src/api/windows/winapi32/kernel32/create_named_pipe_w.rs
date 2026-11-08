use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CreateNamedPipeW(emu: &mut emu::Emu) {
    let name_ptr = arg(emu, 0) as u64;
    let open_mode = arg(emu, 0) as u64;
    let pipe_mode = arg(emu, 2) as u64;
    let instances = arg(emu, 3) as u64;
    let out_buff_sz = arg(emu, 4) as u64;
    let in_buff_sz = arg(emu, 5) as u64;
    let timeout = arg(emu, 6) as u64;
    let security = arg(emu, 7) as u64;

    let name = emu.maps.read_wide_string(name_ptr);

    log_red!(
        emu,
        "kernel32!CreateNamedPipeA  name:{} in: 0x{:x} out: 0x{:x}",
        name,
        in_buff_sz,
        out_buff_sz
    );

    emu.regs_mut().rax = helper::handler_create(&name);
    stdcall_ret(emu, 8);
}
