use crate::emu;

pub fn PathCanonicalizeW(emu: &mut emu::Emu) {
    let dst_ptr = emu.regs().rcx;
    let src_ptr = emu.regs().rdx;
    let src = if src_ptr != 0 {
        emu.maps.read_wide_string(src_ptr)
    } else {
        String::new()
    };
    let mut parts: Vec<&str> = Vec::new();
    for component in src.split(['\\', '/']) {
        match component {
            "." | "" => {}
            ".." => {
                parts.pop();
            }
            _ => parts.push(component),
        }
    }
    let canonical = parts.join("\\");
    let result = if src.starts_with("\\\\") {
        format!("\\\\{}", canonical)
    } else if src.len() >= 2
        && src.as_bytes().get(1) == Some(&b':')
        && src.as_bytes().get(2) == Some(&b'\\')
    {
        format!("{}:\\{}", &src[..1], canonical)
    } else {
        canonical
    };
    if dst_ptr != 0 {
        emu.maps.write_wide_string(dst_ptr, &result);
    }
    log_red!(
        emu,
        "kernelbase!PathCanonicalizeW '{}' => '{}'",
        src,
        result
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
