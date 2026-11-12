use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn toupper(emu: &mut emu::Emu) {
    let c = WinApiAbi::arg(emu, 0) as u32;
    let result = if (0x61..=0x7a).contains(&c) {
        c - 0x20
    } else {
        c
    };

    emu.regs_mut().rax = result as u64;
}
