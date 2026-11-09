use crate::emu;

pub fn PathCombineW(emu: &mut emu::Emu) {
    let dst: u64 = emu.regs().rcx;
    let dir = emu.regs().rdx;
    let file = emu.regs().r8;

    let mut path1 = String::new();
    let mut path2 = String::new();

    if dir > 0 {
        path1 = emu.maps.read_wide_string(dir);
    }
    if file > 0 {
        path2 = emu.maps.read_wide_string(file);
    }

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
}
