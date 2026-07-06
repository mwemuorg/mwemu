use crate::emu;
use crate::winapi::helper;

pub fn CreateNamedPipeA(emu: &mut emu::Emu) {
    let name_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!CreateNamedPipeA cannot read the name_ptr") as u64;
    let open_mode = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!CreateNamedPipeA cannot read the open_mode");
    let pipe_mode = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!CreateNamedPipeA cannot read the pipe_mode");
    let instances = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("kernel32!CreateNamedPipeA cannot read the instances");
    let out_buff_sz = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("kernel32!CreateNamedPipeA cannot read the to_buff_sz");
    let in_buff_sz = emu
        .maps
        .read_dword(emu.regs().get_esp() + 20)
        .expect("kernel32!CreateNamedPipeA cannot read the in_buff_sz");
    let timeout = emu
        .maps
        .read_dword(emu.regs().get_esp() + 24)
        .expect("kernel32!CreateNamedPipeA cannot read the timeout");
    let security = emu
        .maps
        .read_dword(emu.regs().get_esp() + 28)
        .expect("kernel32!CreateNamedPipeA cannot read the security");

    let name = emu.maps.read_string(name_ptr);

    log_red!(
        emu,
        "kernel32!CreateNamedPipeA  name:{} in: 0x{:x} out: 0x{:x}",
        name,
        in_buff_sz,
        out_buff_sz
    );

    for _ in 0..8 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = helper::handler_create(&name);
}
