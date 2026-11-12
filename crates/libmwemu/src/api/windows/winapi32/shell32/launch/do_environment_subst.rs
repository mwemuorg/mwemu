use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn DoEnvironmentSubstA(emu: &mut emu::Emu) {
    do_environment_subst(emu, "DoEnvironmentSubstA", false);
}

pub fn DoEnvironmentSubstW(emu: &mut emu::Emu) {
    do_environment_subst(emu, "DoEnvironmentSubstW", true);
}

fn do_environment_subst(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let s = if str_ptr != 0 {
        strings::read_text(emu, str_ptr, wide)
    } else {
        String::new()
    };
    let arg2 = WinApiAbi::arg(emu, 1) as u32;
    log_red!(emu, "shell32!{api} '{}' arg2:0x{:x}", s, arg2);
    if wide {
        let handle = helper::handler_create("doenvironmentsubstw");
        emu.regs_mut().rax = handle;
        for _ in 0..2u32 {
            emu.stack_pop32(false);
        }
    } else {
        let handle = helper::handler_create("doenvironmentsubsta");
        emu.regs_mut().rax = handle;
        for _ in 0..2u32 {
            emu.stack_pop32(false);
        }
    }
}
