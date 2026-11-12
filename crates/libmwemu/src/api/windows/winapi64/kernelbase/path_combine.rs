use crate::emu;
use crate::winapi::common::strings;

pub fn PathCombineA(emu: &mut emu::Emu) {
    path_combine(emu, "PathCombineA", false);
}

fn path_combine(emu: &mut emu::Emu, api: &str, wide: bool) {
    let dst: u64 = emu.regs().rcx;
    let dir = emu.regs().rdx;
    let file = emu.regs().r8;

    let mut path1 = String::new();
    let mut path2 = String::new();

    if dir > 0 {
        path1 = strings::read_text(emu, dir, wide);
    }
    if file > 0 {
        path2 = strings::read_text(emu, file, wide);
    }

    log_red!(emu, "kernelbase!{api} path1: {} path2: {}", path1, path2);

    if dst != 0 && !path1.is_empty() && !path2.is_empty() {
        strings::write_string(emu, dst, &format!("{}\\{}", path1, path2), wide);
    }

    emu.regs_mut().rax = dst;
}

pub fn PathCombineW(emu: &mut emu::Emu) {
    path_combine(emu, "PathCombineW", true);
}
