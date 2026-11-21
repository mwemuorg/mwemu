use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::trace::input_string;
use crate::winapi::common::{buffers, memory, strings};
use crate::winapi::environment_store::Encoding;
use crate::winapi::libraries::kernel32::file_time::guest_file_time;
use crate::winapi::policy::api_try;
use crate::winapi::registry_store as registry;
use crate::windows::constants::errors::*;

type Guid = [u8; 16];
const TEXT_CAPACITY: usize = "{00000000-0000-0000-0000-000000000000}".len() + 1;

fn guid_text(bytes: Guid) -> String {
    let [
        first @ ..,
        tail0,
        tail1,
        tail2,
        tail3,
        tail4,
        tail5,
        tail6,
        tail7,
    ] = bytes;
    let [a, b, c, d, e, f, g, h] = first;
    let data1 = u32::from_le_bytes([a, b, c, d]);
    let data2 = u16::from_le_bytes([e, f]);
    let data3 = u16::from_le_bytes([g, h]);
    format!(
        "{{{data1:08X}-{data2:04X}-{data3:04X}-{tail0:02X}{tail1:02X}-{tail2:02X}{tail3:02X}{tail4:02X}{tail5:02X}{tail6:02X}{tail7:02X}}}"
    )
}

fn registered_value(emu: &Emu, path: &str) -> Option<String> {
    let root = registry::predefined_path(registry::HKEY_CLASSES_ROOT)?;
    let value = emu.registry.get_value(&registry::join(root, path), "")?;
    if value.kind != registry::REG_SZ || !value.data.len().is_multiple_of(2) {
        return None;
    }
    let units: Vec<_> = value
        .data
        .as_chunks::<2>()
        .0
        .iter()
        .copied()
        .map(u16::from_le_bytes)
        .take_while(|&u| u != 0)
        .collect();
    String::from_utf16(&units).ok()
}

/// Windows writes each completed field before it checks the next separator.
fn parse_fields(
    text: &str,
    bytes: &mut Guid,
    written: &mut usize,
    allow_suffix: bool,
) -> Option<()> {
    let mut input = text.chars();
    if input.next() != Some('{') {
        return None;
    }
    for (index, width) in [4, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1].into_iter().enumerate() {
        if matches!(index, 1 | 2 | 3 | 5) && input.next() != Some('-') {
            return None;
        }
        let mut field = 0u32;
        for _ in 0..2 * width {
            field = (field << 4) | input.next()?.to_digit(16)?;
        }
        bytes[*written..*written + width].copy_from_slice(&field.to_le_bytes()[..width]);
        *written += width;
    }
    (input.next() == Some('}') && (allow_suffix || input.next().is_none())).then_some(())
}

#[derive(Clone, Copy, PartialEq)]
enum Identifier {
    Class,
    Interface,
    ShellClass,
}

fn from_text(emu: &mut Emu, api: &str, kind: Identifier) {
    let call = WinApi(Stdcall);
    let input = api_try!(emu, call.arg(emu, 0));
    let output = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "{api} input={} output={output:#x}",
        input_string(emu, input, Encoding::Wide)
    );
    let mut bytes = [0; 16];
    let mut written = 0;
    let result = if output == 0 || !emu.maps.validate_write_range(output, bytes.len() as u64) {
        HRESULT_E_INVALID_ARG
    } else if input == 0 {
        written = bytes.len();
        S_OK
    } else if let Some(text) = strings::read_text_checked(emu, input, true) {
        if kind == Identifier::Interface && text.encode_utf16().count() != TEXT_CAPACITY - 1 {
            HRESULT_E_INVALID_ARG
        } else {
            let registered =
                if kind == Identifier::Class && !text.is_empty() && !text.starts_with('{') {
                    registered_value(emu, &format!("{text}\\CLSID"))
                } else {
                    None
                };
            let parsed = parse_fields(
                registered.as_deref().unwrap_or(&text),
                &mut bytes,
                &mut written,
                kind == Identifier::ShellClass,
            );
            if kind == Identifier::ShellClass
                || (kind == Identifier::Class && !text.is_empty() && !text.starts_with('{'))
            {
                written = bytes.len();
            }
            if parsed.is_some() {
                S_OK
            } else if kind == Identifier::Interface {
                CO_E_IIDSTRING
            } else {
                CO_E_CLASSSTRING
            }
        }
    } else {
        HRESULT_E_INVALID_ARG
    };
    if written != 0 {
        emu.maps.write_bytes(output, &bytes[..written]);
    }
    call.set_return(emu, result);
    call.cleanup_args(emu, 2);
}

pub fn CLSIDFromString(emu: &mut Emu) {
    from_text(emu, "ole32!CLSIDFromString", Identifier::Class);
}
pub fn IIDFromString(emu: &mut Emu) {
    from_text(emu, "ole32!IIDFromString", Identifier::Interface);
}
pub fn SHCLSIDFromString(emu: &mut Emu) {
    from_text(emu, "shell32!SHCLSIDFromString", Identifier::ShellClass);
}

pub fn StringFromGUID2(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let guid = api_try!(emu, call.arg(emu, 0));
    let output = api_try!(emu, call.arg(emu, 1));
    let capacity = api_try!(emu, call.arg_i32(emu, 2));
    log_red!(
        emu,
        "ole32!StringFromGUID2 guid=0x{:x} capacity={}",
        guid,
        capacity
    );
    let count = if capacity >= TEXT_CAPACITY as i32
        && output != 0
        && emu
            .maps
            .validate_write_range(output, (TEXT_CAPACITY * 2) as u64)
        && memory::read_array(emu, guid).is_some_and(|bytes| {
            emu.maps.write_bytes(
                output,
                &strings::encode_text(&(guid_text(bytes) + "\0"), true),
            )
        }) {
        TEXT_CAPACITY as u64
    } else {
        0
    };
    call.set_return(emu, count);
    call.cleanup_args(emu, 3);
}

fn allocate_text(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let guid = api_try!(emu, call.arg(emu, 0));
    let output = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "{} guid=0x{:x} output=0x{:x}", api, guid, output);
    let result = if output == 0 || !memory::writable_pointer(emu, output) {
        E_POINTER
    } else if let Some(bytes) = memory::read_array(emu, guid) {
        let encoded = strings::encode_text(&(guid_text(bytes) + "\0"), true);
        buffers::return_task_string(emu, output, &encoded)
    } else {
        HRESULT_E_INVALID_ARG
    };
    call.set_return(emu, result);
    call.cleanup_args(emu, 2);
}

pub fn StringFromIID(emu: &mut Emu) {
    allocate_text(emu, "ole32!StringFromIID");
}
pub fn StringFromCLSID(emu: &mut Emu) {
    allocate_text(emu, "ole32!StringFromCLSID");
}

pub fn IsEqualGUID(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let left = api_try!(emu, call.arg(emu, 0));
    let right = api_try!(emu, call.arg(emu, 1));
    let equal = memory::read_array::<16>(emu, left)
        .zip(memory::read_array(emu, right))
        .is_some_and(|(a, b)| a == b);
    log_red!(
        emu,
        "ole32!IsEqualGUID left=0x{:x} right=0x{:x} equal={}",
        left,
        right,
        equal
    );
    call.set_return(emu, u64::from(equal));
    call.cleanup_args(emu, 2);
}

pub fn CoCreateGuid(emu: &mut Emu) {
    const VERSION_4: u8 = 0x40;
    const RFC_VARIANT: u8 = 0x80;
    let call = WinApi(Stdcall);
    let guid = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ole32!CoCreateGuid guid={guid:#x}");
    let status = if guid == 0
        || !emu
            .maps
            .validate_write_range(guid, size_of::<Guid>() as u64)
    {
        HRESULT_E_INVALID_ARG
    } else {
        let mut bytes = [0u8; size_of::<Guid>()];
        for chunk in bytes.as_chunks_mut::<{ size_of::<u64>() }>().0 {
            *chunk = emu.random_u64().to_le_bytes();
        }
        bytes[7] = (bytes[7] & 0x0f) | VERSION_4;
        bytes[8] = (bytes[8] & 0x3f) | RFC_VARIANT;
        if emu.maps.write_bytes(guid, &bytes) {
            S_OK
        } else {
            E_POINTER
        }
    };
    call.set_return(emu, status);
    call.cleanup_args(emu, 1);
}

pub fn CoFileTimeNow(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let file_time = api_try!(emu, call.arg(emu, 0));
    let value = guest_file_time(emu);
    log_red!(emu, "ole32!CoFileTimeNow file_time={file_time:#x}");
    let status = if file_time != 0 && emu.maps.write_bytes(file_time, &value.to_le_bytes()) {
        S_OK
    } else {
        E_POINTER
    };
    call.set_return(emu, status);
    call.cleanup_args(emu, 1);
}

pub fn CoGetCurrentProcess(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let thread = emu.current_thread().id;
    log_red!(emu, "ole32!CoGetCurrentProcess thread={thread}");
    call.set_return(emu, thread);
}

pub fn ProgIDFromCLSID(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [class, output] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "ole32!ProgIDFromCLSID class={class:#x} output={output:#x}"
    );
    let status = if !memory::writable_pointer(emu, output) {
        E_POINTER
    } else if let Some(id) = memory::read_array(emu, class) {
        let key = format!("CLSID\\{}\\ProgID", guid_text(id));
        if let Some(name) = registered_value(emu, &key) {
            buffers::return_task_string(emu, output, &strings::encode_text(&(name + "\0"), true))
        } else {
            memory::write_pointer(emu, output, 0);
            REGDB_E_CLASSNOTREG
        }
    } else {
        HRESULT_E_INVALID_ARG
    };
    call.set_return(emu, status);
    call.cleanup_args(emu, 2);
}
