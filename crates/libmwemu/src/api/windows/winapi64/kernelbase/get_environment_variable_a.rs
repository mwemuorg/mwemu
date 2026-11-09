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
