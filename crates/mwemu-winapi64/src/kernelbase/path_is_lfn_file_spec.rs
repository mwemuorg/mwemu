use crate::emu;

pub fn PathIsLFNFileSpecA(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        emu.maps.read_string(path_ptr)
    } else {
        String::new()
    };
    let filename = path
        .rfind('\\')
        .or_else(|| path.rfind('/'))
        .map(|p| &path[p + 1..])
        .unwrap_or(&path);
    let is_lfn = filename.len() > 12
        || filename.contains(' ')
        || filename.chars().filter(|c| *c == '.').count() > 1;
    log_red!(
        emu,
        "kernelbase!PathIsLFNFileSpecA '{}' => {}",
        path,
        is_lfn
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = if is_lfn { 1 } else { 0 };
}

pub fn PathIsLFNFileSpecW(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        emu.maps.read_wide_string(path_ptr)
    } else {
        String::new()
    };
    let filename = path
        .rfind('\\')
        .or_else(|| path.rfind('/'))
        .map(|p| &path[p + 1..])
        .unwrap_or(&path);
    let is_lfn = filename.len() > 12
        || filename.contains(' ')
        || filename.chars().filter(|c| *c == '.').count() > 1;
    log_red!(
        emu,
        "kernelbase!PathIsLFNFileSpecW '{}' => {}",
        path,
        is_lfn
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = if is_lfn { 1 } else { 0 };
}
