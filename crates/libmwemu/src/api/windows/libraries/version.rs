//! Version queries for guest files and loaded images. Returned views belong to the caller's version block.
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::emu::object_handle::FileHandle;
use crate::winapi::common::{file_names, files, memory, modules, resources, strings};
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;
use crate::windows::constants::files::RT_VERSION;
use crate::windows::constants::files::{FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING};
use crate::windows::constants::security::GENERIC_READ;
use crate::windows::structures::{VS_FIXEDFILEINFO, VS_VERSIONINFO};

const VERSION_RESOURCE_ID: u32 = 1;
const VERSION_HEADER_BYTES: usize = 3 * size_of::<u16>();
const ANSI_VIEW_TAG: &[u8; 4] = b"ANSI";
const STRING_PATH_DEPTH: usize = 3;

struct Node<'a> {
    key: String,
    text: bool,
    count: u16,
    bytes: &'a [u8],
    end: usize,
    value: usize,
    children: usize,
}

impl<'a> Node<'a> {
    fn parse(bytes: &'a [u8], start: usize, limit: usize) -> Option<Self> {
        let remaining = bytes.get(start..limit)?;
        let length = u16::from_le_bytes(remaining.get(..2)?.try_into().ok()?) as usize;
        let block = remaining.get(..length)?;
        let count = u16::from_le_bytes(block.get(2..4)?.try_into().ok()?);
        let kind = u16::from_le_bytes(block.get(4..6)?.try_into().ok()?);
        if kind > 1 {
            return None;
        }
        let mut units = block
            .get(VERSION_HEADER_BYTES..)?
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]));
        let mut key = Vec::new();
        loop {
            let unit = units.next()?;
            if unit == 0 {
                break;
            }
            key.push(unit);
        }
        let value = (VERSION_HEADER_BYTES + (key.len() + 1) * size_of::<u16>())
            .next_multiple_of(size_of::<u32>());
        let bytes_len = count as usize * if kind == 1 { size_of::<u16>() } else { 1 };
        let bytes = if bytes_len == 0 {
            &block[..0]
        } else {
            block.get(value..value + bytes_len)?
        };
        Some(Self {
            key: String::from_utf16(&key).ok()?,
            text: kind == 1,
            count,
            bytes,
            end: start + length,
            value: start + value,
            children: (start + value + bytes_len).next_multiple_of(size_of::<u32>()),
        })
    }

    fn root(bytes: &'a [u8]) -> Option<Self> {
        let root = Self::parse(bytes, 0, bytes.len())?;
        (root.key == VS_VERSIONINFO::KEY
            && !root.text
            && (root.count == 0 || root.bytes.len() == VS_FIXEDFILEINFO::size()))
        .then_some(root)
    }

    fn child(&self, bytes: &'a [u8], key: &str) -> Option<Self> {
        let mut at = self.children;
        while at < self.end {
            let child = Self::parse(bytes, at, self.end)?;
            if child.key.eq_ignore_ascii_case(key) {
                return Some(child);
            }
            at = child.end.next_multiple_of(size_of::<u32>());
        }
        None
    }

    fn ansi_value(&self) -> Vec<u8> {
        let units = self
            .bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>();
        Encoding::Ansi.encode(&units)
    }

    fn fill_ansi_view(&self, bytes: &'a [u8], view: &mut [u8], depth: usize) -> Option<()> {
        if self.text && self.count != 0 {
            let encoded = self.ansi_value();
            view.get_mut(self.value..self.value + encoded.len())?
                .copy_from_slice(&encoded);
        }
        if depth < STRING_PATH_DEPTH {
            let mut at = self.children;
            while at < self.end {
                if self.end - at < size_of::<u16>() || bytes.get(at..at + 2)? == [0, 0] {
                    break;
                }
                let child = Self::parse(bytes, at, self.end)?;
                child.fill_ansi_view(bytes, view, depth + 1)?;
                at = child.end.next_multiple_of(size_of::<u32>());
            }
        }
        Some(())
    }
}

fn loaded_image(emu: &Emu, path: &str) -> Option<u64> {
    if path.is_empty() {
        return None;
    }
    let path = path.replace('/', "\\");
    let qualified = path.contains(['\\', ':']);
    let matches = |absolute: &str| {
        let expected = if qualified {
            absolute
        } else {
            absolute.rsplit('\\').next().unwrap_or(absolute)
        };
        path.eq_ignore_ascii_case(expected)
    };
    if emu.base != 0 && matches(&modules::main_module_path(emu)) {
        return Some(emu.base);
    }
    let leaf = path.rsplit('\\').next()?;
    let module = emu.export_indexes.get_by_name(leaf)?;
    matches(&modules::module_path(emu, module.base)?).then_some(module.base)
}

fn version_bytes(emu: &Emu, path: &str) -> Result<Vec<u8>, u64> {
    if path.is_empty() {
        return Err(ERROR_INVALID_PARAMETER);
    }
    let Some(module) = loaded_image(emu, path) else {
        let units = file_names::absolute_path(&emu.cfg.cwd_path, path.encode_utf16().collect())?;
        let path = String::from_utf16(&units).map_err(|_| ERROR_INVALID_PARAMETER)?;
        let mut file = FileHandle::new(
            path,
            GENERIC_READ,
            OPEN_EXISTING as u32,
            0,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
        )
        .map_err(|error| {
            error
                .downcast_ref::<std::io::Error>()
                .map_or(ERROR_FILE_NOT_FOUND, |error| {
                    if error.kind() == std::io::ErrorKind::NotADirectory {
                        ERROR_FILE_NOT_FOUND
                    } else {
                        files::io_error(error, ERROR_READ_FAULT)
                    }
                })
        })?;
        let mut bytes = resources::file_payload(
            &mut file,
            RT_VERSION,
            VERSION_RESOURCE_ID,
            emu.maps.max_alloc_size,
        )?;
        let len = Node::root(&bytes).ok_or(ERROR_RESOURCE_DATA_NOT_FOUND)?.end;
        bytes.truncate(len);
        return Ok(bytes);
    };
    let (address, size) = resources::numeric_payload(emu, module, RT_VERSION, VERSION_RESOURCE_ID)?;
    let len = emu
        .maps
        .read_word(address)
        .ok_or(ERROR_RESOURCE_DATA_NOT_FOUND)? as usize;
    if len > size as usize {
        return Err(ERROR_RESOURCE_DATA_NOT_FOUND);
    }
    let bytes = emu
        .maps
        .read_bytes_option(address, len)
        .ok_or(ERROR_RESOURCE_DATA_NOT_FOUND)?;
    Node::root(&bytes).ok_or(ERROR_RESOURCE_DATA_NOT_FOUND)?;
    Ok(bytes.into_owned())
}

const FILE_VER_GET_LOCALISED: u32 = 1;
const FILE_VER_GET_NEUTRAL: u32 = 2;
const FILE_VER_GET_PREFETCHED: u32 = 4;

fn version_for_language(emu: &Emu, path: &str, flags: u32) -> Result<Vec<u8>, u64> {
    let base = version_bytes(emu, path)?;
    if flags & (FILE_VER_GET_LOCALISED | FILE_VER_GET_NEUTRAL) == 0
        || path.to_ascii_lowercase().ends_with(".mui")
    {
        return Ok(base);
    }
    let normalized = path.replace('/', "\\");
    let (directory, filename) = normalized
        .rsplit_once('\\')
        .map_or(("", normalized.as_str()), |(dir, name)| (dir, name));
    // The Windows guest profile uses en-US as its UI language.
    let sidecar = if directory.is_empty() {
        format!(r"en-US\{filename}.mui")
    } else {
        format!(r"{directory}\en-US\{filename}.mui")
    };
    let mut localized = match version_bytes(emu, &sidecar) {
        Ok(bytes) => bytes,
        Err(
            ERROR_FILE_NOT_FOUND
            | ERROR_PATH_NOT_FOUND
            | ERROR_RESOURCE_TYPE_NOT_FOUND
            | ERROR_RESOURCE_DATA_NOT_FOUND,
        ) => return Ok(base),
        Err(error) => return Err(error),
    };
    if flags & FILE_VER_GET_NEUTRAL != 0 {
        let fixed = Node::root(&base)
            .ok_or(ERROR_RESOURCE_DATA_NOT_FOUND)?
            .bytes;
        if !fixed.is_empty() {
            let root = Node::root(&localized).ok_or(ERROR_RESOURCE_DATA_NOT_FOUND)?;
            let at = root.value;
            let end = at
                .checked_add(root.bytes.len())
                .filter(|&v| v <= localized.len())
                .ok_or(ERROR_RESOURCE_DATA_NOT_FOUND)?;
            let size = u16::try_from(localized.len() - (end - at) + fixed.len())
                .map_err(|_| ERROR_RESOURCE_DATA_NOT_FOUND)?;
            localized.splice(at..end, fixed.iter().copied());
            localized[..2].copy_from_slice(&size.to_le_bytes());
            localized[2..4].copy_from_slice(&(fixed.len() as u16).to_le_bytes());
        }
    }
    Ok(localized)
}

fn version_info(emu: &mut Emu, api: &str, wide: bool, size_query: bool) {
    let call = WinApi(Stdcall);
    let extended = api.contains("Ex");
    let offset = usize::from(extended);
    let flags = if extended {
        api_try!(emu, call.arg_u32(emu, 0))
    } else {
        FILE_VER_GET_NEUTRAL
    };
    let filename = api_try!(emu, call.arg(emu, offset));
    let handle = api_try!(emu, call.arg(emu, offset + 1));
    let transfer = if size_query {
        None
    } else {
        Some((
            api_try!(emu, call.arg_u32(emu, offset + 2)) as usize,
            api_try!(emu, call.arg(emu, offset + 3)),
        ))
    };
    let result = (|| {
        if size_query && handle != 0 && !emu.maps.write_dword(handle, 0) {
            return Err(ERROR_NOACCESS);
        }
        if flags & !(FILE_VER_GET_LOCALISED | FILE_VER_GET_NEUTRAL | FILE_VER_GET_PREFETCHED) != 0 {
            return Err(ERROR_NOT_SUPPORTED);
        }
        let path = file_names::read_path(emu, filename, wide)?;
        log_red!(emu, "version!{api} filename={path:?}");
        let bytes = version_for_language(emu, &path, flags)?;
        // Keep narrow strings in a second view so A and W pointers both remain valid.
        let size = bytes.len() * 2 + ANSI_VIEW_TAG.len();
        let Some((capacity, output)) = transfer else {
            return Ok(size as u64);
        };
        if capacity == 0 || output == 0 {
            return Err(ERROR_INVALID_PARAMETER);
        }
        let root = Node::root(&bytes).ok_or(ERROR_RESOURCE_DATA_NOT_FOUND)?;
        let mut view = vec![0; bytes.len()];
        root.fill_ansi_view(&bytes, &mut view, 0)
            .ok_or(ERROR_RESOURCE_DATA_NOT_FOUND)?;
        let mut block = bytes;
        block.extend_from_slice(ANSI_VIEW_TAG);
        block.extend_from_slice(&view);
        let count = capacity.min(size);
        if !emu.maps.validate_write_range(output, count as u64)
            || !emu.maps.write_bytes(output, &block[..count])
        {
            return Err(ERROR_NOACCESS);
        }
        Ok(1)
    })();
    let value = result.unwrap_or_else(|error| {
        emu.set_last_error(error);
        0
    });
    call.set_return(emu, value);
    call.cleanup_args(emu, offset + if size_query { 2 } else { 4 });
}

fn value_view(emu: &Emu, block: u64, path: &str, wide: bool) -> Option<(u64, u32)> {
    let len = emu.maps.read_word(block)? as usize;
    let bytes = emu.maps.read_bytes_option(block, len)?;
    let mut node = Node::root(&bytes)?;
    if path != "\\" {
        let keys = path.strip_prefix('\\')?.split('\\');
        for key in keys {
            if key.is_empty() {
                return None;
            }
            node = node.child(&bytes, key)?;
        }
    }
    let offset = node.value as u64;
    if node.count == 0 || (node.text && !node.bytes.ends_with(&[0, 0])) {
        return None;
    }
    if wide || !node.text {
        return Some((block.checked_add(offset)?, node.count as u32));
    }
    let tag = block.checked_add(len as u64)?;
    if emu
        .maps
        .read_bytes_option(tag, ANSI_VIEW_TAG.len())?
        .as_ref()
        != ANSI_VIEW_TAG
    {
        return None;
    }
    let encoded = node.ansi_value();
    let pointer = tag
        .checked_add(ANSI_VIEW_TAG.len() as u64)?
        .checked_add(offset)?;
    (emu.maps.read_bytes_option(pointer, encoded.len())?.as_ref() == encoded)
        .then_some((pointer, encoded.len() as u32))
}

fn query(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let block = api_try!(emu, call.arg(emu, 0));
    let subblock = api_try!(emu, call.arg(emu, 1));
    let output = api_try!(emu, call.arg(emu, 2));
    let length = api_try!(emu, call.arg(emu, 3));
    let path = strings::read_text_checked(emu, subblock, wide);
    log_red!(
        emu,
        "version!{api} block={block:#x} subblock={:?} buffer_out={output:#x} length_out={length:#x}",
        path.as_deref()
    );
    let value = path.and_then(|path| value_view(emu, block, &path, wide));
    let length_writable = emu
        .maps
        .validate_write_range(length, size_of::<u32>() as u64);
    let success = length_writable
        && memory::writable_pointer(emu, output)
        && value.is_some_and(|(ptr, count)| {
            memory::write_pointer(emu, output, ptr) && emu.maps.write_dword(length, count)
        });
    if !success && length_writable {
        emu.maps.write_dword(length, 0);
    }
    call.set_return(emu, success as u64);
    call.cleanup_args(emu, 4);
}

pub(crate) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "GetFileVersionInfoSizeA" | "GetFileVersionInfoSizeExA" => version_info(emu, api, false, true),
        "GetFileVersionInfoSizeW" | "GetFileVersionInfoSizeExW" => version_info(emu, api, true, true),
        "GetFileVersionInfoA" | "GetFileVersionInfoExA" => version_info(emu, api, false, false),
        "GetFileVersionInfoW" | "GetFileVersionInfoExW" => version_info(emu, api, true, false),
        "VerQueryValueA" => query(emu, api, false),
        "VerQueryValueW" => query(emu, api, true),
    }, handled(()) => true, fallback => false)
}
