use crate::emu;
pub fn GetEnvironmentVariableA(emu: &mut emu::Emu) {
    let lp_name = emu.regs().rcx;
    let lp_buffer = emu.regs().rdx;
    let n_size = emu.regs().r8 as u32;
    let name = emu.maps.read_string(lp_name);

    let value = match name.to_uppercase().as_str() {
        "SYSTEMROOT" | "WINDIR" => "C:\\Windows".to_string(),
        "SYSTEMDRIVE" => "C:".to_string(),
        "COMSPEC" => "C:\\Windows\\system32\\cmd.exe".to_string(),
        "TEMP" | "TMP" => emu.cfg.temp_path.clone(),
        "PATH" => "C:\\Windows\\system32;C:\\Windows;C:\\Windows\\System32\\Wbem".to_string(),
        "PATHEXT" => ".COM;.EXE;.BAT;.CMD;.VBS;.VBE;.JS;.JSE;.WSF;.WSH;.MSC".to_string(),
        "USERNAME" => emu.cfg.user_name.clone(),
        "USERPROFILE" => format!("C:\\Users\\{}", emu.cfg.user_name),
        "APPDATA" => format!("C:\\Users\\{}\\AppData\\Roaming", emu.cfg.user_name),
        "LOCALAPPDATA" => format!("C:\\Users\\{}\\AppData\\Local", emu.cfg.user_name),
        "PROGRAMFILES" => "C:\\Program Files".to_string(),
        "PROGRAMFILES(X86)" => "C:\\Program Files (x86)".to_string(),
        "COMPUTERNAME" => emu.cfg.host_name.clone(),
        "NUMBER_OF_PROCESSORS" => "4".to_string(),
        "PROCESSOR_ARCHITECTURE" => "AMD64".to_string(),
        "OS" => "Windows_NT".to_string(),
        _ => {
            log_red!(
                emu,
                "kernel32!GetEnvironmentVariableA '{}' => not found",
                name
            );
            emu.set_last_error(203);
            emu.regs_mut().rax = 0;
            return;
        }
    };

    let bytes_needed = value.len() as u32 + 1;

    log_red!(
        emu,
        "kernel32!GetEnvironmentVariableA '{}' = '{}'",
        name,
        value
    );

    if n_size == 0 || lp_buffer == 0 {
        emu.regs_mut().rax = bytes_needed as u64;
        return;
    }

    if n_size < bytes_needed {
        emu.regs_mut().rax = bytes_needed as u64;
        return;
    }

    emu.maps.write_string(lp_buffer, &value);
    emu.regs_mut().rax = (bytes_needed - 1) as u64;
}
