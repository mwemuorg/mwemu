use crate::emu;

pub fn toupper(emu: &mut emu::Emu) {
    let c = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let result = if (0x61..=0x7a).contains(&c) {
        c - 0x20
    } else {
        c
    };
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = result as u64;
}
