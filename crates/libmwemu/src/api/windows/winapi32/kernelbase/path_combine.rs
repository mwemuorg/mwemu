use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn PathCombineA(emu: &mut emu::Emu) {
    path_combine(emu, "PathCombineA", false);
}

fn path_combine(emu: &mut emu::Emu, api: &str, wide: bool) {
    let dst = arg(emu, 0) as u64;
    let dir = arg(emu, 1) as u64;
    let file = arg(emu, 2) as u64;
    let path1 = if dir != 0 {
        strings::read_text(emu, dir, wide)
    } else {
        String::new()
    };
    let path2 = if file != 0 {
        strings::read_text(emu, file, wide)
    } else {
        String::new()
    };
    log_red!(emu, "kernelbase!{api} path1: {} path2: {}", path1, path2);
    if dst != 0 && !path1.is_empty() && !path2.is_empty() {
        strings::write_string(emu, dst, &format!("{}\\{}", path1, path2), wide);
    }
    emu.regs_mut().rax = dst;
    stdcall_ret(emu, 3);
}

pub(super) fn PathCombineW(emu: &mut emu::Emu) {
    path_combine(emu, "PathCombineW", true);
}
