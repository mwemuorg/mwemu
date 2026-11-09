use crate::emu;

pub fn GetEnvironmentVariableA(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().rcx;
    let buf_ptr = emu.regs().rdx;
    let buf_size = emu.regs().r8;
    let name = emu.maps.read_string(name_ptr);
    let name_upper = name.to_uppercase();
    let value = match name_upper.as_str() {
        "PATH" => Some("C:\\Windows\\system32;C:\\Windows"),
        "TEMP" | "TMP" => Some("C:\\Users\\User\\AppData\\Local\\Temp"),
        "USERPROFILE" => Some("C:\\Users\\User"),
        "SYSTEMROOT" | "WINDIR" => Some("C:\\Windows"),
        "SYSTEMDRIVE" => Some("C:"),
        "COMPUTERNAME" => Some("DESKTOP-MWEMU"),
        "USERNAME" => Some("User"),
        "APPDATA" => Some("C:\\Users\\User\\AppData\\Roaming"),
        "LOCALAPPDATA" => Some("C:\\Users\\User\\AppData\\Local"),
        "PROGRAMFILES" => Some("C:\\Program Files"),
        "PROGRAMDATA" => Some("C:\\ProgramData"),
        "HOMEDRIVE" => Some("C:"),
        "HOMEPATH" => Some("\\Users\\User"),
        "COMSPEC" => Some("C:\\Windows\\system32\\cmd.exe"),
        "NUMBER_OF_PROCESSORS" => Some("4"),
        "PROCESSOR_ARCHITECTURE" => Some("AMD64"),
        "OS" => Some("Windows_NT"),
        _ => None,
    };
    if let Some(val) = value {
        log_red!(
            emu,
            "kernelbase!GetEnvironmentVariableA '{}' => '{}'",
            name,
            val
        );
        if buf_ptr != 0 && buf_size > val.len() as u64 {
            emu.maps.write_string(buf_ptr, val);
            emu.regs_mut().rax = val.len() as u64;
        } else {
            emu.regs_mut().rax = val.len() as u64 + 1;
        }
    } else {
        log_red!(
            emu,
            "kernelbase!GetEnvironmentVariableA '{}' => NOT FOUND",
            name
        );
        emu.set_last_error(203); // ERROR_ENVVAR_NOT_FOUND
        emu.regs_mut().rax = 0;
    }
}

pub fn GetEnvironmentVariableW(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().rcx;
    let buf_ptr = emu.regs().rdx;
    let buf_size = emu.regs().r8;
    let name = emu.maps.read_wide_string(name_ptr);
    let value = match name.to_uppercase().as_str() {
        "SYSTEMROOT" | "WINDIR" => "C:\\Windows".to_string(),
        "TEMP" | "TMP" => "C:\\Users\\user\\AppData\\Local\\Temp".to_string(),
        "PROGRAMFILES" => "C:\\Program Files".to_string(),
        "PROGRAMFILES(X86)" => "C:\\Program Files (x86)".to_string(),
        "COMSPEC" => "C:\\Windows\\system32\\cmd.exe".to_string(),
        "USERPROFILE" => "C:\\Users\\user".to_string(),
        "APPDATA" => "C:\\Users\\user\\AppData\\Roaming".to_string(),
        "LOCALAPPDATA" => "C:\\Users\\user\\AppData\\Local".to_string(),
        "SYSTEMDRIVE" => "C:".to_string(),
        "PATH" => "C:\\Windows\\system32;C:\\Windows".to_string(),
        "COMPUTERNAME" => "DESKTOP-MWEMU".to_string(),
        "USERNAME" => "user".to_string(),
        "OS" => "Windows_NT".to_string(),
        "NUMBER_OF_PROCESSORS" => "4".to_string(),
        "PROCESSOR_ARCHITECTURE" => "x86".to_string(),
        _ => String::new(),
    };
    if value.is_empty() {
        emu.set_last_error(203);
        emu.regs_mut().rax = 0;
    } else if buf_ptr != 0 && buf_size > 0 {
        emu.maps.write_wide_string(buf_ptr, &value);
        emu.regs_mut().rax = value.len() as u64;
    } else {
        emu.regs_mut().rax = value.len() as u64 + 1;
    }
    log_red!(
        emu,
        "kernelbase!GetEnvironmentVariableW '{}' => '{}'",
        name,
        value
    );
}
