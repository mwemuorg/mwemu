use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::file_names::absolute_path;
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::api_try;
use crate::winapi::profile_store::WIN_INI;
use crate::windows::constants::errors::*;
use crate::windows::constants::system::WINDOWS_DIRECTORY;

pub(crate) fn profile(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let codec = Encoding::from_wide(api.ends_with('W'));
    let private = api.contains("PrivateProfile");
    let writing = api.starts_with("Write");
    let integer = api.contains("ProfileInt");
    let section = api_try!(emu, call.arg(emu, 0));
    let key = api_try!(emu, call.arg(emu, 1));
    let value = api_try!(emu, call.arg(emu, 2));
    let string_output = !writing && !integer;
    let output = if string_output {
        api_try!(emu, call.arg(emu, 3))
    } else {
        0
    };
    let capacity = if string_output {
        api_try!(emu, call.arg_u32(emu, 4)) as usize
    } else {
        0
    };
    let file_index = if string_output { 5 } else { 3 };
    let file = if private {
        api_try!(emu, call.arg(emu, file_index))
    } else {
        0
    };
    let result = (|| {
        let section = optional_string(emu, codec, section)?;
        let key = optional_string(emu, codec, key)?;
        let path = if file == 0 {
            WIN_INI.encode_utf16().collect()
        } else {
            codec.read(emu, file)?
        };
        let file = String::from_utf16(&absolute_path(WINDOWS_DIRECTORY, path)?)
            .map_err(|_| ERROR_INVALID_PARAMETER)?;
        log_red!(
            emu,
            "{library}!{api} file={file:?} section={section:?} key={key:?}"
        );
        if writing {
            let value = optional_string(emu, codec, value)?;
            if let Some(section) = section {
                emu.profiles
                    .write(&file, &section, key.as_deref(), value.as_deref());
                return Ok(1);
            }
            return Ok(0); // A cache flush returns zero.
        }
        let entry = section
            .as_ref()
            .zip(key.as_ref())
            .and_then(|(s, k)| emu.profiles.read(&file, s, k));
        if integer {
            return Ok(u64::from(entry.map_or(value as u32, profile_integer)));
        }
        if capacity == 0 {
            return Ok(0);
        }
        if section.is_none() || key.is_none() {
            let names = emu.profiles.names(&file, section.as_deref());
            let mut units = Vec::new();
            for name in names {
                units.extend(name.encode_utf16());
                units.push(0);
            }
            return copy_profile(emu, codec, output, capacity, &units, true);
        }
        let text = match entry {
            Some(s) => {
                let bytes = s.as_bytes();
                if bytes.len() >= 2
                    && matches!(bytes[0], b'\'' | b'"')
                    && bytes.first() == bytes.last()
                {
                    s[1..s.len() - 1].to_owned()
                } else {
                    s.to_owned()
                }
            }
            None => optional_string(emu, codec, value)?
                .unwrap_or_default()
                .trim_end_matches(' ')
                .to_owned(),
        };
        copy_profile(
            emu,
            codec,
            output,
            capacity,
            &text.encode_utf16().collect::<Vec<_>>(),
            false,
        )
    })();
    let result = result.unwrap_or_else(|err| {
        emu.set_last_error(err);
        0
    });
    call.set_return(emu, result);
    call.cleanup_args(emu, file_index + usize::from(private));
}

fn optional_string(emu: &Emu, codec: Encoding, ptr: u64) -> Result<Option<String>, u64> {
    if ptr == 0 {
        return Ok(None);
    }
    String::from_utf16(&codec.read(emu, ptr)?)
        .map(Some)
        .map_err(|_| ERROR_INVALID_PARAMETER)
}

fn copy_profile(
    emu: &mut Emu,
    codec: Encoding,
    output: u64,
    capacity: usize,
    units: &[u16],
    list: bool,
) -> Result<u64, u64> {
    let width = codec.width();
    let mut bytes = codec.encode(units);
    let count = bytes.len() / width;
    let truncated_list = list && count >= capacity;
    let copied = if truncated_list {
        capacity.saturating_sub(2)
    } else {
        count.min(capacity - 1)
    };
    let terminators = if truncated_list || (list && count == 0) {
        2.min(capacity)
    } else {
        1
    };
    bytes.truncate(copied * width);
    bytes.resize((copied + terminators) * width, 0);
    if output == 0
        || !emu.maps.validate_write_range(output, bytes.len() as u64)
        || !emu.maps.write_bytes(output, &bytes)
    {
        return Err(ERROR_NOACCESS);
    }
    Ok(copied as u64)
}

fn profile_integer(text: &str) -> u32 {
    let value = text.trim_start();
    let negative = value.starts_with('-');
    let value = value.trim_start_matches(['+', '-']);
    let (base, digits) = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .map_or((10, value), |rest| (16, rest));
    let parsed = digits
        .chars()
        .map_while(|ch| ch.to_digit(base))
        .fold(0u32, |n, digit| n.wrapping_mul(base).wrapping_add(digit));
    if negative {
        parsed.wrapping_neg()
    } else {
        parsed
    }
}
