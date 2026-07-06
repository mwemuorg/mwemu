use crate::emu;
use crate::serialization;
use crate::winapi::winapi64;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "RealShellExecuteA" => RealShellExecuteA(emu),
        "SHGetFolderPathW" => SHGetFolderPathW(emu),
        "ShellExecuteA" => ShellExecuteA(emu),
        "ShellExecuteW" => ShellExecuteW(emu),
        _ => {
            if !emu.cfg.skip_unimplemented {
                if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                    serialization::Serialization::dump(
                        emu,
                        emu.cfg.dump_filename.as_ref().unwrap(),
                    );
                }

                unimplemented!("atemmpt to call unimplemented API 0x{:x} {}", addr, api);
            }
            log::warn!(
                "calling unimplemented API 0x{:x} {} at 0x{:x}",
                addr,
                api,
                emu.regs().rip
            );
            return api.to_ascii_lowercase();
        }
    }
    String::new()
}

fn ShellExecuteA(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let lp_operation = emu.regs().rdx;
    let lp_file = emu.regs().r8;
    let lp_parameters = emu.regs().r9;
    let lp_directory = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("shell32!ShellExecuteA error reading lp_directory");
    let n_show_cmd = emu
        .maps
        .read_qword(emu.regs().rsp + 0x28)
        .expect("shell32!ShellExecuteA error reading n_show_cmd");

    let operation = if lp_operation != 0 {
        emu.maps.read_string(lp_operation)
    } else {
        "open".to_string()
    };
    let file = emu.maps.read_string(lp_file);
    let params = if lp_parameters != 0 {
        emu.maps.read_string(lp_parameters)
    } else {
        "".to_string()
    };

    log_red!(
        emu,
        "shell32!ShellExecuteA op: {} file: {} params: {}",
        operation,
        file,
        params
    );

    emu.regs_mut().rax = 42; // > 32 means success
}

fn ShellExecuteW(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let lp_operation = emu.regs().rdx;
    let lp_file = emu.regs().r8;
    let lp_parameters = emu.regs().r9;
    let lp_directory = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("shell32!ShellExecuteW error reading lp_directory");
    let n_show_cmd = emu
        .maps
        .read_qword(emu.regs().rsp + 0x28)
        .expect("shell32!ShellExecuteW error reading n_show_cmd");

    let operation = if lp_operation != 0 {
        emu.maps.read_wide_string(lp_operation)
    } else {
        "open".to_string()
    };
    let file = emu.maps.read_wide_string(lp_file);
    let params = if lp_parameters != 0 {
        emu.maps.read_wide_string(lp_parameters)
    } else {
        "".to_string()
    };

    log_red!(
        emu,
        "shell32!ShellExecuteW op: {} file: {} params: {}",
        operation,
        file,
        params
    );

    emu.regs_mut().rax = 42; // > 32 means success
}

fn RealShellExecuteA(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let operation = emu.regs().rdx;
    let file_ptr = emu.regs().r8;
    let params_ptr = emu.regs().r9;
    let dir = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("cannot read parameter");
    let bShowWindow = emu
        .maps
        .read_qword(emu.regs().rsp + 0x28)
        .expect("cannot read parameter");

    let file = emu.maps.read_string(file_ptr);
    let params = emu.maps.read_string(params_ptr);

    log_red!(emu, "shell32!RealShellExecuteA {} {}", file, params);

    emu.regs_mut().rax = 34;
}

fn SHGetFolderPathW(emu: &mut emu::Emu) {
    /*
    HRESULT SHGetFolderPathW(
        [in]  HWND   hwnd,
        [in]  int    csidl,
        [in]  HANDLE hToken,
        [in]  DWORD  dwFlags,
        [out] LPWSTR pszPath
    );
    */
    let hwnd = emu.regs().rcx;
    let csidl = emu.regs().rdx as i32;
    let h_token = emu.regs().r8;
    let dw_flags = emu.regs().r9 as u32;
    let psz_path = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("shell32!SHGetFolderPathW error reading pszPath");

    log_red!(
        emu,
        "shell32!SHGetFolderPathW hwnd: 0x{:x} csidl: 0x{:x} token: 0x{:x} flags: 0x{:x} path: 0x{:x}",
        hwnd,
        csidl,
        h_token,
        dw_flags,
        psz_path
    );

    // Simple folder path mapping
    let folder_path = match csidl & 0xFFFF {
        0x0005 => "C:\\Users\\User\\Documents", // CSIDL_PERSONAL
        0x001a => "C:\\Users\\User\\AppData\\Roaming", // CSIDL_APPDATA
        0x001c => "C:\\Users\\User\\AppData\\Local", // CSIDL_LOCAL_APPDATA
        0x0020 => "C:\\Users\\User\\AppData\\Local\\Microsoft\\Windows\\INetCache", // CSIDL_INTERNET_CACHE
        0x0021 => "C:\\Users\\User\\AppData\\Local\\Microsoft\\Windows\\INetCookies", // CSIDL_COOKIES
        0x0022 => "C:\\Users\\User\\AppData\\Local\\Microsoft\\Windows\\History", // CSIDL_HISTORY
        0x0023 => "C:\\ProgramData",           // CSIDL_COMMON_APPDATA
        0x0024 => "C:\\Windows",               // CSIDL_WINDOWS
        0x0025 => "C:\\Windows\\System32",     // CSIDL_SYSTEM
        0x0026 => "C:\\Program Files",         // CSIDL_PROGRAM_FILES
        0x0027 => "C:\\Users\\User\\Pictures", // CSIDL_MYPICTURES
        0x0028 => "C:\\Users\\User",           // CSIDL_PROFILE
        0x0029 => "C:\\Windows\\SysWOW64",     // CSIDL_SYSTEMX86
        0x002a => "C:\\Program Files (x86)",   // CSIDL_PROGRAM_FILESX86
        0x002b => "C:\\Program Files\\Common Files", // CSIDL_PROGRAM_FILES_COMMON
        0x002c => "C:\\Program Files (x86)\\Common Files", // CSIDL_PROGRAM_FILES_COMMONX86
        0x002d => "C:\\ProgramData\\Microsoft\\Windows\\Templates", // CSIDL_COMMON_TEMPLATES
        0x002e => "C:\\Users\\Public\\Documents", // CSIDL_COMMON_DOCUMENTS
        0x002f => "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs\\Administrative Tools", // CSIDL_COMMON_ADMINTOOLS
        0x0030 => {
            "C:\\Users\\User\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Administrative Tools"
        } // CSIDL_ADMINTOOLS
        0x0031 => "C:\\Users\\User\\AppData\\Roaming\\Microsoft\\Windows\\Network Shortcuts", // CSIDL_CONNECTIONS
        0x0035 => "C:\\Users\\Public\\Music", // CSIDL_COMMON_MUSIC
        0x0036 => "C:\\Users\\Public\\Pictures", // CSIDL_COMMON_PICTURES
        0x0037 => "C:\\Users\\Public\\Videos", // CSIDL_COMMON_VIDEO
        0x0038 => "C:\\Windows\\Resources",   // CSIDL_RESOURCES
        _ => panic!("Unimplemented CSIDL value: 0x{:x} at {}", csidl, emu.pos),
    };

    if psz_path != 0 {
        emu.maps.write_wide_string(psz_path, folder_path);
    }

    // Return S_OK (0x00000000)
    emu.regs_mut().rax = 0;
}
