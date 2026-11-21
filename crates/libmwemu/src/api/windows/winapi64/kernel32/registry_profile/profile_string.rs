use crate::api::abi::windows::{ArgumentError, CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::winapi::profile_store;

// Profile ("INI") APIs over profile_store. The non-private variants work on
// win.ini; a NULL lpFileName does the same for the private ones.

/*
DWORD GetPrivateProfileStringW(
  [in]  LPCWSTR lpAppName,
  [in]  LPCWSTR lpKeyName,
  [in]  LPCWSTR lpDefault,
  [out] LPWSTR  lpReturnedString,
  [in]  DWORD   nSize,
  [in]  LPCWSTR lpFileName
);
*/
pub fn GetPrivateProfileStringA(emu: &mut emu::Emu) {
    get_string(emu, "GetPrivateProfileStringA", false, true);
}

pub fn GetPrivateProfileStringW(emu: &mut emu::Emu) {
    get_string(emu, "GetPrivateProfileStringW", true, true);
}

pub fn GetProfileStringA(emu: &mut emu::Emu) {
    get_string(emu, "GetProfileStringA", false, false);
}

pub fn GetProfileStringW(emu: &mut emu::Emu) {
    get_string(emu, "GetProfileStringW", true, false);
}

/*
BOOL WritePrivateProfileStringW(
  [in] LPCWSTR lpAppName,
  [in] LPCWSTR lpKeyName,
  [in] LPCWSTR lpString,
  [in] LPCWSTR lpFileName
);
*/
pub fn WritePrivateProfileStringA(emu: &mut emu::Emu) {
    write_string(emu, "WritePrivateProfileStringA", false, true);
}

pub fn WritePrivateProfileStringW(emu: &mut emu::Emu) {
    write_string(emu, "WritePrivateProfileStringW", true, true);
}

pub fn WriteProfileStringA(emu: &mut emu::Emu) {
    write_string(emu, "WriteProfileStringA", false, false);
}

pub fn WriteProfileStringW(emu: &mut emu::Emu) {
    write_string(emu, "WriteProfileStringW", true, false);
}

/*
UINT GetPrivateProfileIntW(
  [in] LPCWSTR lpAppName,
  [in] LPCWSTR lpKeyName,
  [in] INT     nDefault,
  [in] LPCWSTR lpFileName
);
*/
pub fn GetPrivateProfileIntA(emu: &mut emu::Emu) {
    get_int(emu, "GetPrivateProfileIntA", false, true);
}

pub fn GetPrivateProfileIntW(emu: &mut emu::Emu) {
    get_int(emu, "GetPrivateProfileIntW", true, true);
}

pub fn GetProfileIntA(emu: &mut emu::Emu) {
    get_int(emu, "GetProfileIntA", false, false);
}

pub fn GetProfileIntW(emu: &mut emu::Emu) {
    get_int(emu, "GetProfileIntW", true, false);
}

/// Shared body of [Get]ProfileString / GetPrivateProfileString: copies the
/// stored value (or lpDefault) truncated to nSize - 1 characters plus the NUL,
/// and returns the characters copied.
fn get_string(emu: &mut emu::Emu, api: &str, wide: bool, private: bool) {
    let call = WinApi(Stdcall);
    let section = opt_text(emu, api_try!(emu, call.arg(emu, 0)), wide);
    let key = opt_text(emu, api_try!(emu, call.arg(emu, 1)), wide);
    let default = opt_text(emu, api_try!(emu, call.arg(emu, 2)), wide).unwrap_or_default();
    let out = api_try!(emu, call.arg(emu, 3));
    let size = api_try!(emu, call.arg(emu, 4)) & 0xffff_ffff;
    let file = api_try!(emu, profile_file(emu, private, wide));

    // section / key enumeration (NULL names) is not modelled: answer the default
    let value = match (&section, &key) {
        (Some(section), Some(key)) => profile_store::read(&file, section, key),
        _ => None,
    }
    .unwrap_or(default);

    log_red!(
        emu,
        "kernel32!{} [{}] {:?}/{:?} => '{}'",
        api,
        file,
        section,
        key,
        value
    );

    if out == 0 || size == 0 {
        call.set_return(emu, 0);
        return;
    }
    let copied: String = value.chars().take(size as usize - 1).collect();
    write_text_z(emu, out, &copied, wide);
    call.set_return(emu, copied.chars().count() as u64);
}

/// Shared body of WriteProfileString / WritePrivateProfileString.
fn write_string(emu: &mut emu::Emu, api: &str, wide: bool, private: bool) {
    let call = WinApi(Stdcall);
    let section = opt_text(emu, api_try!(emu, call.arg(emu, 0)), wide);
    let key = opt_text(emu, api_try!(emu, call.arg(emu, 1)), wide);
    let value = opt_text(emu, api_try!(emu, call.arg(emu, 2)), wide);
    let file = if private {
        opt_text(emu, api_try!(emu, call.arg(emu, 3)), wide)
            .unwrap_or_else(|| profile_store::WIN_INI.to_string())
    } else {
        profile_store::WIN_INI.to_string()
    };

    log_red!(
        emu,
        "kernel32!{} [{}] {:?}/{:?} = {:?}",
        api,
        file,
        section,
        key,
        value
    );

    // a NULL section only flushes the cached file
    if let Some(section) = section {
        profile_store::write(&file, &section, key.as_deref(), value.as_deref());
    }
    call.set_return(emu, 1);
}

/// Shared body of GetProfileInt / GetPrivateProfileInt: nDefault when the key
/// is missing, else the leading decimal digits of the value (0 if none).
fn get_int(emu: &mut emu::Emu, api: &str, wide: bool, private: bool) {
    let call = WinApi(Stdcall);
    let section = opt_text(emu, api_try!(emu, call.arg(emu, 0)), wide);
    let key = opt_text(emu, api_try!(emu, call.arg(emu, 1)), wide);
    let default = api_try!(emu, call.arg(emu, 2)) & 0xffff_ffff;
    let file = if private {
        opt_text(emu, api_try!(emu, call.arg(emu, 3)), wide)
            .unwrap_or_else(|| profile_store::WIN_INI.to_string())
    } else {
        profile_store::WIN_INI.to_string()
    };

    let stored = match (&section, &key) {
        (Some(section), Some(key)) => profile_store::read(&file, section, key),
        _ => None,
    };
    let value = match &stored {
        Some(text) => parse_leading_int(text),
        None => default,
    };

    log_red!(
        emu,
        "kernel32!{} [{}] {:?}/{:?} => {}",
        api,
        file,
        section,
        key,
        value
    );
    call.set_return(emu, value);
}

/// Filename argument for private profile APIs, win.ini otherwise.
fn profile_file(emu: &mut emu::Emu, private: bool, wide: bool) -> Result<String, ArgumentError> {
    let call = WinApi(Stdcall);
    let ptr = if private { call.arg(emu, 5)? } else { 0 };
    Ok(opt_text(emu, ptr, wide).unwrap_or_else(|| profile_store::WIN_INI.to_string()))
}

fn parse_leading_int(text: &str) -> u64 {
    let trimmed = text.trim_start();
    let (negative, digits) = match trimmed.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, trimmed),
    };
    let digits: String = digits.chars().take_while(|c| c.is_ascii_digit()).collect();
    let value = digits.parse::<i64>().unwrap_or(0);
    (if negative { -value } else { value }) as u32 as u64
}

fn opt_text(emu: &mut emu::Emu, ptr: u64, wide: bool) -> Option<String> {
    if ptr == 0 {
        None
    } else {
        Some(strings::read_text(emu, ptr, wide))
    }
}

/// `text` plus its NUL terminator, UTF-16LE or ANSI.
fn write_text_z(emu: &mut emu::Emu, addr: u64, text: &str, wide: bool) {
    let bytes: Vec<u8> = if wide {
        text.encode_utf16()
            .chain(std::iter::once(0))
            .flat_map(|c| c.to_le_bytes())
            .collect()
    } else {
        text.bytes().chain(std::iter::once(0)).collect()
    };
    emu.maps.write_bytes(addr, &bytes);
}
