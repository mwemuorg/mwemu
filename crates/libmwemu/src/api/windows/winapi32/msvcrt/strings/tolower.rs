use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn tolower(emu: &mut emu::Emu) {
    let c = WinApiAbi::arg(emu, 0) as u32;
    let result = if (0x41..=0x5a).contains(&c) {
        c + 0x20
    } else {
        c
    };

    emu.regs_mut().rax = result as u64;
}
