use crate::emu;
use crate::winapi::common::strings;

pub fn PathIsSameRootA(emu: &mut emu::Emu) {
    path_is_same_root(emu, "PathIsSameRootA", false);
}

fn path_is_same_root(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path1_ptr = emu.regs().rcx;
    let path2_ptr = emu.regs().rdx;
    let path1 = if path1_ptr != 0 {
        strings::read_text(emu, path1_ptr, wide)
    } else {
        String::new()
    };
    let path2 = if path2_ptr != 0 {
        strings::read_text(emu, path2_ptr, wide)
    } else {
        String::new()
    };
    let root1 = path1.get(..3).unwrap_or("").to_lowercase();
    let root2 = path2.get(..3).unwrap_or("").to_lowercase();
    let same = !root1.is_empty() && root1 == root2;
    log_red!(
        emu,
        "kernelbase!{api} '{}' vs '{}' => {}",
        path1,
        path2,
        same
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = if same { 1 } else { 0 };
}

pub fn PathIsSameRootW(emu: &mut emu::Emu) {
    path_is_same_root(emu, "PathIsSameRootW", true);
}
