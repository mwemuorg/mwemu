use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn PathCanonicalizeA(emu: &mut emu::Emu) {
    path_canonicalize(emu, "PathCanonicalizeA", false);
}

pub fn PathCanonicalizeW(emu: &mut emu::Emu) {
    path_canonicalize(emu, "PathCanonicalizeW", true);
}

fn path_canonicalize(emu: &mut emu::Emu, api: &str, wide: bool) {
    let dst_ptr = emu.regs().rcx;
    let src_ptr = emu.regs().rdx;
    let src = if src_ptr != 0 {
        strings::read_text(emu, src_ptr, wide)
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
    if wide {
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
            strings::write_string(emu, dst_ptr, &result, wide);
        }
        log_red!(emu, "kernelbase!{api} '{}' => '{}'", src, result);
    } else {
        let result = if src.starts_with("\\\\") {
            format!("\\\\{}", canonical)
        } else if src.len() >= 3
            && src.as_bytes().get(1) == Some(&b':')
            && src.as_bytes().get(2) == Some(&b'\\')
        {
            format!("{}:\\{}", &src[..1], canonical)
        } else {
            canonical
        };
        if dst_ptr != 0 {
            strings::write_string(emu, dst_ptr, &result, wide);
        }
        log_red!(emu, "kernelbase!{api} '{}' => '{}'", src, result);
        let _h = helper::handler_create("dir_entry");
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
