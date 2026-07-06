use crate::emu;

pub fn lstrlen(emu: &mut emu::Emu) {
    let s_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!lstrlen cannot read string") as u64;

    emu.stack_pop32(false);
    let s = emu.maps.read_string(s_ptr);
    let len = s.len() as u64;

    log_red!(emu, "kernel32!lstrlen '{}' ={}", s, len);

    emu.regs_mut().rax = len;
}
