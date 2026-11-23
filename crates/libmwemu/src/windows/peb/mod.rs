use std::mem::offset_of;

use goblin::pe::header::{SIZEOF_COFF_HEADER, SIZEOF_PE_MAGIC};
use goblin::pe::optional_header::{
    SIZEOF_STANDARD_FIELDS_32, SIZEOF_STANDARD_FIELDS_64, SIZEOF_WINDOWS_FIELDS_32,
    SIZEOF_WINDOWS_FIELDS_64, StandardFields64, WindowsFields32, WindowsFields64,
};

use crate::emu::Emu;
use crate::maps::Maps;
use crate::winapi::common::memory::{pointer_size, read_pointer, write_pointer};
use crate::windows::structures::{LdrDataTableEntry64, Peb, PebLdrData};

pub mod peb32;
pub mod peb64;

const OPTIONAL_HEADER_OFFSET: usize = SIZEOF_PE_MAGIC + SIZEOF_COFF_HEADER;
const ENTRY_POINT_RVA_OFFSET: u64 =
    (OPTIONAL_HEADER_OFFSET + offset_of!(StandardFields64, address_of_entry_point)) as u64;
const IMAGE_SIZE_OFFSET32: u64 = (OPTIONAL_HEADER_OFFSET
    + SIZEOF_STANDARD_FIELDS_32
    + offset_of!(WindowsFields32, size_of_image)) as u64;
const IMAGE_SIZE_OFFSET64: u64 = (OPTIONAL_HEADER_OFFSET
    + SIZEOF_STANDARD_FIELDS_64
    + offset_of!(WindowsFields64, size_of_image)) as u64;
const EXPORT_DIRECTORY_OFFSET32: u64 =
    (OPTIONAL_HEADER_OFFSET + SIZEOF_STANDARD_FIELDS_32 + SIZEOF_WINDOWS_FIELDS_32) as u64;
const EXPORT_DIRECTORY_OFFSET64: u64 =
    (OPTIONAL_HEADER_OFFSET + SIZEOF_STANDARD_FIELDS_64 + SIZEOF_WINDOWS_FIELDS_64) as u64;

// Reserve space for loader-owned UTF-16 names, including names replaced after startup.
const LDR_NAME_RESERVE: u64 = 0x840;

struct LdrNames<'a> {
    full: String,
    base: &'a str,
    full_capacity: u16,
    base_capacity: u16,
}

impl<'a> LdrNames<'a> {
    fn new(path: &'a str) -> Option<Self> {
        let (full, base) = match path.rsplit_once(['\\', '/']) {
            Some((_, base)) => (path.to_owned(), base),
            None => (format!(r"C:\Windows\System32\{path}"), path),
        };
        let capacity = |name: &str| u16::try_from((name.encode_utf16().count() + 1) * 2).ok();
        Some(Self {
            full_capacity: capacity(&full)?,
            base_capacity: capacity(base)?,
            full,
            base,
        })
    }

    fn storage_size(&self) -> u64 {
        u64::from(self.full_capacity) + u64::from(self.base_capacity)
    }
}

/// Remove a loader-owned entry from the load, memory and initialization lists.
pub(crate) fn unlink_dynamic_entry(emu: &mut Emu, entry: u64) {
    let width = pointer_size(emu);
    let list_size = 2 * width;
    let head32 = if width == 4 {
        emu.maps
            .get_map_by_name("peb")
            .and_then(|peb| emu.maps.read_dword(peb.get_base() + Peb::LDR_OFFSET))
            .map(u64::from)
    } else {
        None
    };
    let mut unlink = |offset: u64, header: Option<u64>| {
        let Some((node, blink)) = entry
            .checked_add(offset)
            .and_then(|node| Some((node, node.checked_add(width)?)))
        else {
            return;
        };
        let (Some(next), Some(prev)) = (read_pointer(emu, node), read_pointer(emu, blink)) else {
            return;
        };
        let Some(next_blink) = next.checked_add(width) else {
            return;
        };
        if next != 0
            && prev != 0
            && read_pointer(emu, prev) == Some(node)
            && read_pointer(emu, next_blink) == Some(node)
        {
            write_pointer(emu, prev, next);
            write_pointer(emu, next_blink, prev);
            if let Some(head) = header {
                if read_pointer(emu, head) == Some(node) {
                    write_pointer(emu, head, next);
                }
                if read_pointer(emu, head + width) == Some(node) {
                    write_pointer(emu, head + width, prev);
                }
            }
        }
    };
    for offset in [0, list_size, list_size * 2] {
        unlink(
            offset,
            head32.map(|ldr| ldr + PebLdrData::IN_LOAD_ORDER_MODULE_LIST_OFFSET + offset),
        );
    }
    if width == 8 {
        unlink(LdrDataTableEntry64::HASH_LINKS_OFFSET, None);
    }
    emu.maps.dealloc(entry);
}

fn map_wide_string(maps: &mut Maps, name: &str, text: &str, capacity: u64) -> u64 {
    let Some(map) = maps.alloc_rw_pages(name, capacity) else {
        log::error!("cannot allocate {capacity} bytes for process parameter {name}");
        return 0;
    };
    let base = map.get_base();
    map.write_wide_string(base, text);
    base
}

/// Build the guest command line from its image path and the configured raw arguments.
pub(crate) fn command_line(image: &str, arguments: &str) -> String {
    let mut line = if image.contains([' ', '\t']) {
        format!("\"{image}\"")
    } else {
        image.to_owned()
    };
    if !arguments.is_empty() && !arguments.starts_with([' ', '\t']) {
        line.push(' ');
    }
    line.push_str(arguments);
    line
}
