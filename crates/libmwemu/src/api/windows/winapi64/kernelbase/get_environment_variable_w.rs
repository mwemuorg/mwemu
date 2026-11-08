use crate::emu;

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
