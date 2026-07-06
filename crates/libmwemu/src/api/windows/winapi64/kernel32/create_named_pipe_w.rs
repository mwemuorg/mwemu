use crate::emu;
use crate::winapi::helper;

pub fn CreateNamedPipeW(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().rcx;
    let open_mode = emu.regs().rcx;
    let pipe_mode = emu.regs().r8;
    let instances = emu.regs().r9;
    let out_buff_sz = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("kernel32!CreateNamedPipeA cannot read the to_buff_sz");
    let in_buff_sz = emu
        .maps
        .read_qword(emu.regs().rsp + 0x28)
        .expect("kernel32!CreateNamedPipeA cannot read the in_buff_sz");
    let timeout = emu
        .maps
        .read_qword(emu.regs().rsp + 0x30)
        .expect("kernel32!CreateNamedPipeA cannot read the timeout");
    let security = emu
        .maps
        .read_qword(emu.regs().rsp + 0x38)
        .expect("kernel32!CreateNamedPipeA cannot read the security");

    let name = emu.maps.read_wide_string(name_ptr);

    log_red!(
        emu,
        "kernel32!CreateNamedPipeA  name:{} in: 0x{:x} out: 0x{:x}",
        name,
        in_buff_sz,
        out_buff_sz
    );

    emu.regs_mut().rax = helper::handler_create(&name);
}
