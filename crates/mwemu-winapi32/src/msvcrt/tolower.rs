use crate::emu;

pub fn tolower(emu: &mut emu::Emu) {
    let c = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let result = if (0x41..=0x5a).contains(&c) {
        c + 0x20
    } else {
        c
    };
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = result as u64;
}
