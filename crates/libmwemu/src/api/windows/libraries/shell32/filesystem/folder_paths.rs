use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::{buffers, memory, strings, trace};
use crate::winapi::environment_store::{Encoding, PROFILE};
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;
use crate::windows::constants::files::MAX_PATH;
use crate::windows::constants::shell::*;

const DEFAULT_USER_TOKEN: u64 = 0xffff_ffff_ffff_ffff;
const DEFAULT_USER_PROFILE: &str = r"C:\Users\Default";
const PROGRAM_DATA: &str = r"C:\ProgramData";
const PUBLIC_PROFILE: &str = r"C:\Users\Public";
const PROGRAM_FILES: &str = r"C:\Program Files";
const PROGRAM_FILES_X86: &str = r"C:\Program Files (x86)";

pub fn SHGetFolderPathA(emu: &mut Emu) {
    get_folder_path(emu, "SHGetFolderPathA", false);
}

pub fn SHGetFolderPathW(emu: &mut Emu) {
    get_folder_path(emu, "SHGetFolderPathW", true);
}

fn get_folder_path(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let _owner = api_try!(emu, call.arg(emu, 0));
    let folder = api_try!(emu, call.arg_u32(emu, 1));
    let token = api_try!(emu, call.arg(emu, 2));
    let kind = api_try!(emu, call.arg_u32(emu, 3));
    let output = api_try!(emu, call.arg(emu, 4));
    let path = folder_path(emu, folder, token, kind);
    let status = write_folder_path(emu, output, path, wide);
    log_red!(
        emu,
        "{api} folder={folder:#x} token={token:#x} kind={kind} status={status:#x}"
    );
    call.cleanup_args(emu, 5);
    call.set_return(emu, status);
}

pub fn SHGetSpecialFolderPathA(emu: &mut Emu) {
    get_special_folder_path(emu, "SHGetSpecialFolderPathA", false);
}

pub fn SHGetSpecialFolderPathW(emu: &mut Emu) {
    get_special_folder_path(emu, "SHGetSpecialFolderPathW", true);
}

fn get_special_folder_path(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let _owner = api_try!(emu, call.arg(emu, 0));
    let output = api_try!(emu, call.arg(emu, 1));
    let folder = api_try!(emu, call.arg_u32(emu, 2));
    let _create = api_try!(emu, call.arg_u32(emu, 3));
    let path = folder_path(emu, folder, 0, SHGFP_TYPE_CURRENT);
    let status = write_folder_path(emu, output, path, wide);
    log_red!(emu, "{api} folder={folder:#x} status={status:#x}");
    call.cleanup_args(emu, 4);
    call.set_return(emu, u64::from(status == S_OK));
}

fn write_folder_path(emu: &mut Emu, output: u64, path: Result<String, u64>, wide: bool) -> u64 {
    let terminator = strings::encode_text("\0", wide);
    if output == 0 || !emu.maps.write_bytes(output, &terminator) {
        return E_POINTER;
    }
    let path = match path {
        Ok(path) => path,
        Err(status) => return status,
    };
    let encoded = strings::encode_text(&format!("{path}\0"), wide);
    if encoded.len() / terminator.len() > MAX_PATH {
        return E_FAIL;
    }
    if !emu.maps.write_bytes(output, &encoded) {
        return E_POINTER;
    }
    S_OK
}

fn folder_path(emu: &Emu, folder: u32, token: u64, kind: u32) -> Result<String, u64> {
    if kind != SHGFP_TYPE_CURRENT && kind != SHGFP_TYPE_DEFAULT {
        return Err(HRESULT_E_INVALID_ARG);
    }
    let profile = if token == 0 {
        PROFILE
    } else if token == DEFAULT_USER_TOKEN || token == DEFAULT_USER_TOKEN as u32 as u64 {
        DEFAULT_USER_PROFILE
    } else {
        return Err(E_NOTIMPL);
    };
    let allowed_flags = CSIDL_FLAG_CREATE
        | CSIDL_FLAG_DONT_VERIFY
        | CSIDL_FLAG_DONT_UNEXPAND
        | CSIDL_FLAG_NO_ALIAS
        | CSIDL_FLAG_PER_USER_INIT;
    if folder & CSIDL_FLAG_MASK & !allowed_flags != 0 {
        return Err(HRESULT_E_INVALID_ARG);
    }
    let folder = folder & !CSIDL_FLAG_MASK;
    let roaming = format!(r"{profile}\AppData\Roaming");
    let local = format!(r"{profile}\AppData\Local");
    let start_menu = format!(r"{roaming}\Microsoft\Windows\Start Menu");
    let common_menu = format!(r"{PROGRAM_DATA}\Microsoft\Windows\Start Menu");
    Ok(match folder {
        CSIDL_DESKTOP | CSIDL_DESKTOPDIRECTORY => format!(r"{profile}\Desktop"),
        CSIDL_PROGRAMS => format!(r"{start_menu}\Programs"),
        CSIDL_PERSONAL => format!(r"{profile}\Documents"),
        CSIDL_FAVORITES => format!(r"{profile}\Favorites"),
        CSIDL_STARTUP | CSIDL_ALTSTARTUP => format!(r"{start_menu}\Programs\Startup"),
        CSIDL_RECENT => format!(r"{roaming}\Microsoft\Windows\Recent"),
        CSIDL_SENDTO => format!(r"{roaming}\Microsoft\Windows\SendTo"),
        CSIDL_STARTMENU => start_menu,
        CSIDL_MYMUSIC => format!(r"{profile}\Music"),
        CSIDL_MYVIDEO => format!(r"{profile}\Videos"),
        CSIDL_NETHOOD => format!(r"{roaming}\Microsoft\Windows\Network Shortcuts"),
        CSIDL_FONTS => format!(r"{}\Fonts", emu.cfg.windows_directory),
        CSIDL_TEMPLATES => format!(r"{roaming}\Microsoft\Windows\Templates"),
        CSIDL_COMMON_STARTMENU => common_menu,
        CSIDL_COMMON_PROGRAMS => format!(r"{common_menu}\Programs"),
        CSIDL_COMMON_STARTUP | CSIDL_COMMON_ALTSTARTUP => {
            format!(r"{common_menu}\Programs\Startup")
        }
        CSIDL_COMMON_DESKTOPDIRECTORY => format!(r"{PUBLIC_PROFILE}\Desktop"),
        CSIDL_APPDATA => roaming,
        CSIDL_PRINTHOOD => format!(r"{roaming}\Microsoft\Windows\Printer Shortcuts"),
        CSIDL_LOCAL_APPDATA => local,
        CSIDL_INTERNET_CACHE => format!(r"{local}\Microsoft\Windows\INetCache"),
        CSIDL_COOKIES => format!(r"{local}\Microsoft\Windows\INetCookies"),
        CSIDL_HISTORY => format!(r"{local}\Microsoft\Windows\History"),
        CSIDL_COMMON_APPDATA => PROGRAM_DATA.into(),
        CSIDL_WINDOWS => emu.cfg.windows_directory.clone(),
        CSIDL_SYSTEM => emu.cfg.system_directory.clone(),
        CSIDL_PROGRAM_FILES => PROGRAM_FILES.into(),
        CSIDL_MYPICTURES => format!(r"{profile}\Pictures"),
        CSIDL_PROFILE => profile.into(),
        CSIDL_SYSTEMX86 => {
            if emu.cfg.arch.is_64bits() {
                format!(r"{}\SysWOW64", emu.cfg.windows_directory)
            } else {
                emu.cfg.system_directory.clone()
            }
        }
        CSIDL_PROGRAM_FILESX86 => if emu.cfg.arch.is_64bits() {
            PROGRAM_FILES_X86
        } else {
            PROGRAM_FILES
        }
        .into(),
        CSIDL_PROGRAM_FILES_COMMON => format!(r"{PROGRAM_FILES}\Common Files"),
        CSIDL_PROGRAM_FILES_COMMONX86 => format!(
            r"{}\Common Files",
            if emu.cfg.arch.is_64bits() {
                PROGRAM_FILES_X86
            } else {
                PROGRAM_FILES
            }
        ),
        CSIDL_COMMON_TEMPLATES => format!(r"{PROGRAM_DATA}\Microsoft\Windows\Templates"),
        CSIDL_COMMON_DOCUMENTS => format!(r"{PUBLIC_PROFILE}\Documents"),
        CSIDL_COMMON_ADMINTOOLS => format!(r"{common_menu}\Programs\Administrative Tools"),
        CSIDL_ADMINTOOLS => format!(r"{start_menu}\Programs\Administrative Tools"),
        CSIDL_COMMON_MUSIC => format!(r"{PUBLIC_PROFILE}\Music"),
        CSIDL_COMMON_PICTURES => format!(r"{PUBLIC_PROFILE}\Pictures"),
        CSIDL_COMMON_VIDEO => format!(r"{PUBLIC_PROFILE}\Videos"),
        CSIDL_RESOURCES => format!(r"{}\Resources", emu.cfg.windows_directory),
        CSIDL_CDBURN_AREA => format!(r"{local}\Microsoft\Windows\Burn\Burn"),
        CSIDL_INTERNET | CSIDL_CONTROLS | CSIDL_PRINTERS | CSIDL_BITBUCKET | CSIDL_DRIVES
        | CSIDL_NETWORK | CSIDL_CONNECTIONS => return Err(S_FALSE),
        _ => return Err(HRESULT_E_INVALID_ARG),
    })
}

fn known_folder(emu: &Emu, id: u64, flags: u32, token: u64) -> Result<String, u64> {
    let bytes = memory::read_array(emu, id).ok_or(E_POINTER)?;
    let csidl = match u128::from_le_bytes(bytes) {
        FOLDER_ID_DESKTOP => CSIDL_DESKTOPDIRECTORY,
        FOLDER_ID_DOCUMENTS => CSIDL_PERSONAL,
        FOLDER_ID_LOCAL_APP_DATA => CSIDL_LOCAL_APPDATA,
        FOLDER_ID_ROAMING_APP_DATA => CSIDL_APPDATA,
        FOLDER_ID_WINDOWS => CSIDL_WINDOWS,
        FOLDER_ID_SYSTEM => CSIDL_SYSTEM,
        FOLDER_ID_SYSTEM_X86 => CSIDL_SYSTEMX86,
        FOLDER_ID_PROFILE => CSIDL_PROFILE,
        FOLDER_ID_PROGRAM_DATA => CSIDL_COMMON_APPDATA,
        FOLDER_ID_PROGRAM_FILES => CSIDL_PROGRAM_FILES,
        FOLDER_ID_PROGRAM_FILES_X86 => CSIDL_PROGRAM_FILESX86,
        FOLDER_ID_PROGRAM_FILES_COMMON => CSIDL_PROGRAM_FILES_COMMON,
        FOLDER_ID_PROGRAM_FILES_COMMON_X86 => CSIDL_PROGRAM_FILES_COMMONX86,
        FOLDER_ID_PICTURES => CSIDL_MYPICTURES,
        FOLDER_ID_MUSIC => CSIDL_MYMUSIC,
        FOLDER_ID_VIDEOS => CSIDL_MYVIDEO,
        FOLDER_ID_FAVORITES => CSIDL_FAVORITES,
        FOLDER_ID_STARTUP => CSIDL_STARTUP,
        FOLDER_ID_START_MENU => CSIDL_STARTMENU,
        FOLDER_ID_PROGRAMS => CSIDL_PROGRAMS,
        FOLDER_ID_RECENT => CSIDL_RECENT,
        FOLDER_ID_SEND_TO => CSIDL_SENDTO,
        FOLDER_ID_FONTS => CSIDL_FONTS,
        FOLDER_ID_TEMPLATES => CSIDL_TEMPLATES,
        FOLDER_ID_PUBLIC_DESKTOP => CSIDL_COMMON_DESKTOPDIRECTORY,
        FOLDER_ID_PUBLIC_DOCUMENTS => CSIDL_COMMON_DOCUMENTS,
        FOLDER_ID_PUBLIC_PICTURES => CSIDL_COMMON_PICTURES,
        FOLDER_ID_PUBLIC_MUSIC => CSIDL_COMMON_MUSIC,
        FOLDER_ID_PUBLIC_VIDEOS => CSIDL_COMMON_VIDEO,
        _ => return Err(E_NOTIMPL),
    };
    let supported = KF_FLAG_DONT_VERIFY | KF_FLAG_DEFAULT_PATH;
    if flags & !supported != 0 {
        return Err(E_NOTIMPL);
    }
    let kind = if flags & KF_FLAG_DEFAULT_PATH != 0 {
        SHGFP_TYPE_DEFAULT
    } else {
        SHGFP_TYPE_CURRENT
    };
    folder_path(emu, csidl, token, kind)
}

pub fn SHGetKnownFolderPath(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [id, flags, token, output] = api_try!(emu, call.args(emu));
    let status = if output == 0 || !memory::write_pointer(emu, output, 0) {
        E_POINTER
    } else {
        match known_folder(emu, id, flags as u32, token) {
            Ok(path) => buffers::return_task_string(
                emu,
                output,
                &strings::encode_text(&format!("{path}\0"), true),
            ),
            Err(status) => status,
        }
    };
    log_red!(
        emu,
        "shell32!SHGetKnownFolderPath folder_id={id:#x} flags={flags:#x} token={token:#x} path_out={output:#x} status={status:#x}"
    );
    call.set_return(emu, status);
    call.cleanup_args(emu, 4);
}

pub fn SHGetFolderPathEx(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [id, flags, token, output, capacity] = api_try!(emu, call.args(emu));
    let status = match known_folder(emu, id, flags as u32, token) {
        Err(status) => status,
        Ok(_) if output == 0 => E_POINTER,
        Ok(path) => {
            let encoded = strings::encode_text(&format!("{path}\0"), true);
            if encoded.len() / 2 > capacity as u32 as usize {
                HRESULT_INSUFFICIENT_BUFFER
            } else if emu.maps.write_bytes(output, &encoded) {
                S_OK
            } else {
                E_POINTER
            }
        }
    };
    log_red!(
        emu,
        "shell32!SHGetFolderPathEx folder_id={id:#x} flags={flags:#x} token={token:#x} path={output:#x} capacity={capacity} status={status:#x}"
    );
    call.set_return(emu, status);
    call.cleanup_args(emu, 5);
}

fn subdirectory(emu: &mut Emu, api: &str, encoding: Encoding) {
    let call = WinApi(Stdcall);
    let [_owner, id, token, kind, subdir, output] = api_try!(emu, call.args(emu));
    let path = folder_path(emu, id as u32, token, kind as u32).and_then(|mut base| {
        if subdir != 0 {
            let units = encoding.read(emu, subdir).map_err(|_| E_POINTER)?;
            let text = String::from_utf16(&units).map_err(|_| HRESULT_E_INVALID_ARG)?;
            if !text.is_empty() {
                base.push('\\');
                base.push_str(text.trim_start_matches(['\\', '/']));
            }
        }
        Ok(base)
    });
    let status = write_folder_path(emu, output, path, matches!(encoding, Encoding::Wide));
    log_red!(
        emu,
        "shell32!{api} folder={id:#x} token={token:#x} kind={kind} subdirectory={subdirectory} status={status:#x}",
        subdirectory = trace::input_string(emu, subdir, encoding)
    );
    call.set_return(emu, status);
    call.cleanup_args(emu, 6);
}

pub fn SHGetFolderPathAndSubDirA(emu: &mut Emu) {
    subdirectory(emu, "SHGetFolderPathAndSubDirA", Encoding::Ansi);
}

pub fn SHGetFolderPathAndSubDirW(emu: &mut Emu) {
    subdirectory(emu, "SHGetFolderPathAndSubDirW", Encoding::Wide);
}
