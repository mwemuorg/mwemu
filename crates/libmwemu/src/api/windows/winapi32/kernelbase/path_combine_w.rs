use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn PathCombineW(emu: &mut emu::Emu) {
    let dst = arg(emu, 0) as u64;
    let dir = arg(emu, 1) as u64;
    let file = arg(emu, 2) as u64;
    let path1 = if dir != 0 {
        emu.maps.read_wide_string(dir)
    } else {
        String::new()
    };
    let path2 = if file != 0 {
        emu.maps.read_wide_string(file)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernelbase!PathCombineW path1: {} path2: {}",
        path1,
        path2
    );
    if dst != 0 && !path1.is_empty() && !path2.is_empty() {
        emu.maps
            .write_wide_string(dst, &format!("{}\\{}", path1, path2));
    }
    emu.regs_mut().rax = dst;
    stdcall_ret(emu, 3);
}
