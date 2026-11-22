use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::*;

fn absolute(url: &str) -> Option<(&str, &str)> {
    let after = url
        .strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))?;
    let end = after
        .find('/')
        .map_or(url.len(), |i| url.len() - after.len() + i);
    (!after.is_empty() && !after.starts_with('/')).then(|| (&url[..end], &url[end..]))
}

fn canonical(url: &str) -> Option<String> {
    if !url
        .bytes()
        .all(|ch| ch.is_ascii_alphanumeric() || b"-._~:/".contains(&ch))
    {
        return None;
    }
    let (root, path) = absolute(url)?;
    if path.contains("//") {
        return None;
    }
    let mut parts = Vec::new();
    for component in path.split('/') {
        match component {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            _ => parts.push(component),
        }
    }
    let mut value = format!("{root}/{}", parts.join("/"));
    if !parts.is_empty() && (path.ends_with('/') || path.ends_with("/.") || path.ends_with("/..")) {
        value.push('/');
    }
    Some(value)
}

fn combine(base: &str, fragment: &str) -> Option<String> {
    let (origin, _) = absolute(base)?;
    if absolute(fragment).is_some() {
        return canonical(fragment);
    }
    if fragment.starts_with("//") || fragment.is_empty() {
        return None;
    }
    if fragment.starts_with('/') {
        return canonical(&format!("{origin}{fragment}"));
    }
    let directory = base[origin.len()..]
        .rsplit_once('/')
        .map_or("", |(directory, _)| directory);
    canonical(&format!("{origin}{directory}/{fragment}"))
}

fn url(emu: &mut Emu, api: &str, wide: bool, join: bool) {
    let call = WinApi(Stdcall);
    let codec = Encoding::from_wide(wide);
    let first = api_try!(emu, call.arg(emu, 0));
    let output_slot = if join { 2 } else { 1 };
    let second = if join {
        api_try!(emu, call.arg(emu, 1))
    } else {
        0
    };
    let output = api_try!(emu, call.arg(emu, output_slot));
    let length = api_try!(emu, call.arg(emu, output_slot + 1));
    let flags = api_try!(emu, call.arg_u32(emu, output_slot + 2));
    let decode = |ptr| String::from_utf16(&codec.read(emu, ptr).ok()?).ok();
    let value = if flags != 0 {
        None
    } else if join {
        decode(first)
            .zip(decode(second))
            .and_then(|(base, fragment)| combine(&base, &fragment))
    } else {
        decode(first).and_then(|url| canonical(&url))
    };
    let Some(value) = value else {
        policy::unsupported(emu, "wininet", api, emu.pc());
        return;
    };
    let units: Vec<_> = value.encode_utf16().chain([0]).collect();
    let bytes = codec.encode(&units);
    let needed = units.len() as u32;
    let status = match emu.maps.read_dword(length) {
        None => Err(ERROR_INVALID_PARAMETER),
        Some(capacity) if capacity < needed || output == 0 => {
            if emu.maps.write_dword(length, needed) {
                Err(ERROR_INSUFFICIENT_BUFFER)
            } else {
                Err(ERROR_INVALID_PARAMETER)
            }
        }
        Some(_)
            if emu.maps.validate_write_range(length, 4) && emu.maps.write_bytes(output, &bytes) =>
        {
            emu.maps.write_dword(length, needed - 1);
            Ok(1)
        }
        Some(_) => Err(ERROR_INVALID_PARAMETER),
    };
    log_red!(
        emu,
        "wininet!{api} url={value:?} output={output:#x} length={length:#x} flags={flags:#x}"
    );
    super::result(emu, output_slot + 3, status);
}

pub fn InternetCanonicalizeUrlA(emu: &mut Emu) {
    url(emu, "InternetCanonicalizeUrlA", false, false);
}
pub fn InternetCanonicalizeUrlW(emu: &mut Emu) {
    url(emu, "InternetCanonicalizeUrlW", true, false);
}
pub fn InternetCombineUrlA(emu: &mut Emu) {
    url(emu, "InternetCombineUrlA", false, true);
}
pub fn InternetCombineUrlW(emu: &mut Emu) {
    url(emu, "InternetCombineUrlW", true, true);
}
