use crate::emu;

pub fn PathIsSameRootA(emu: &mut emu::Emu) {
    let path1_ptr = emu.regs().rcx;
    let path2_ptr = emu.regs().rdx;
    let path1 = if path1_ptr != 0 {
        emu.maps.read_string(path1_ptr)
    } else {
        String::new()
    };
    let path2 = if path2_ptr != 0 {
        emu.maps.read_string(path2_ptr)
    } else {
        String::new()
    };
    let root1 = path1.get(..3).unwrap_or("").to_lowercase();
    let root2 = path2.get(..3).unwrap_or("").to_lowercase();
    let same = !root1.is_empty() && root1 == root2;
    log_red!(
        emu,
        "kernelbase!PathIsSameRootA '{}' vs '{}' => {}",
        path1,
        path2,
        same
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = if same { 1 } else { 0 };
}

pub fn PathIsSameRootW(emu: &mut emu::Emu) {
    let path1_ptr = emu.regs().rcx;
    let path2_ptr = emu.regs().rdx;
    let path1 = if path1_ptr != 0 {
        emu.maps.read_wide_string(path1_ptr)
    } else {
        String::new()
    };
    let path2 = if path2_ptr != 0 {
        emu.maps.read_wide_string(path2_ptr)
    } else {
        String::new()
    };
    let root1 = path1.get(..3).unwrap_or("").to_lowercase();
    let root2 = path2.get(..3).unwrap_or("").to_lowercase();
    let same = !root1.is_empty() && root1 == root2;
    log_red!(
        emu,
        "kernelbase!PathIsSameRootW '{}' vs '{}' => {}",
        path1,
        path2,
        same
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = if same { 1 } else { 0 };
}
