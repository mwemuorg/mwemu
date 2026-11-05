use crate::emu;

pub fn ExpandEnvironmentStringsW(emu: &mut emu::Emu) {
    let src_ptr = emu.regs().rcx;
    let dst_ptr = emu.regs().rdx;
    let size = emu.regs().r8 as u32;

    let src = emu.maps.read_wide_string(src_ptr);

    let expanded = expand_env_vars(emu, &src);

    log_red!(
        emu,
        "kernel32!ExpandEnvironmentStringsW '{}' => '{}'",
        src,
        expanded
    );

    let needed = expanded.chars().count() as u32 + 1;

    if size == 0 || dst_ptr == 0 {
        emu.regs_mut().rax = needed as u64;
        return;
    }

    if size >= needed {
        emu.maps.write_wide_string(dst_ptr, &expanded);
    }

    emu.regs_mut().rax = needed as u64;
}

fn expand_env_vars(emu: &emu::Emu, src: &str) -> String {
    let mut result = src.to_string();
    let replacements = [
        ("%SYSTEMROOT%", "C:\\Windows"),
        ("%WINDIR%", "C:\\Windows"),
        ("%SYSTEMDRIVE%", "C:"),
        ("%COMSPEC%", "C:\\Windows\\system32\\cmd.exe"),
        ("%PROGRAMFILES%", "C:\\Program Files"),
        ("%PROGRAMFILES(X86)%", "C:\\Program Files (x86)"),
        ("%COMMONPROGRAMFILES%", "C:\\Program Files\\Common Files"),
        ("%NUMBER_OF_PROCESSORS%", "4"),
        ("%PROCESSOR_ARCHITECTURE%", "AMD64"),
        ("%OS%", "Windows_NT"),
    ];
    for (var, val) in &replacements {
        result = result.replace(var, val);
        let upper = var.to_uppercase();
        if upper != *var {
            result = result.replace(&upper, val);
        }
    }
    let user = emu.cfg.user_name.clone();
    result = result.replace("%USERNAME%", &user);
    result = result.replace("%USERPROFILE%", &format!("C:\\Users\\{}", user));
    result = result.replace(
        "%APPDATA%",
        &format!("C:\\Users\\{}\\AppData\\Roaming", user),
    );
    result = result.replace(
        "%LOCALAPPDATA%",
        &format!("C:\\Users\\{}\\AppData\\Local", user),
    );
    result = result.replace("%TEMP%", &emu.cfg.temp_path);
    result = result.replace("%TMP%", &emu.cfg.temp_path);
    result = result.replace("%COMPUTERNAME%", &emu.cfg.host_name);
    result
}
