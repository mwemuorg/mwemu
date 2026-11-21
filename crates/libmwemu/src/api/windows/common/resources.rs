//! Resource lookup in loaded guest images. External MUI files are not modeled.

use crate::emu::object_handle::FileHandle;
use goblin::pe::section_table::{SIZEOF_SECTION_TABLE, SectionTable};
use std::io::SeekFrom;

use goblin::pe::header::{
    CoffHeader, DOS_MAGIC, Header, PE_MAGIC, PE_POINTER_OFFSET, SIZEOF_COFF_HEADER, SIZEOF_PE_MAGIC,
};
use rs_header::pe::resource::{Resource, ResourceDirectory, ResourceError, ResourceKey};
use rs_header::pe::structures::ImageResourceDataEntry32;

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::common::memory::{read_guest_bytes, writable_pointer, write_pointer};
use crate::winapi::common::strings;
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;
use crate::windows::constants::files::RT_STRING;
use crate::windows::constants::nls::EN_US_LOCALE;

struct ResourceImage {
    base: u64,
    size: u64,
    directory: u64,
    directory_size: usize,
}

impl ResourceImage {
    fn load(emu: &Emu, module: u64) -> Result<Self, u64> {
        let base = if module == 0 { emu.base } else { module };
        if base == 0 {
            return Err(ERROR_MOD_NOT_FOUND);
        }
        if emu.maps.read_word(base) != Some(DOS_MAGIC) {
            return Err(ERROR_BAD_EXE_FORMAT);
        }
        let pointer_address = base
            .checked_add(PE_POINTER_OFFSET as u64)
            .ok_or(ERROR_BAD_EXE_FORMAT)?;
        let pe_pointer = emu
            .maps
            .read_dword(pointer_address)
            .ok_or(ERROR_BAD_EXE_FORMAT)?;
        let nt_address = base
            .checked_add(pe_pointer as u64)
            .ok_or(ERROR_BAD_EXE_FORMAT)?;
        let nt_size = SIZEOF_PE_MAGIC + SIZEOF_COFF_HEADER;
        let nt = emu
            .maps
            .read_bytes_option(nt_address, nt_size)
            .ok_or(ERROR_BAD_EXE_FORMAT)?;
        let mut offset = SIZEOF_PE_MAGIC;
        let coff = CoffHeader::parse(&nt, &mut offset).map_err(|_| ERROR_BAD_EXE_FORMAT)?;
        let header_size = (pe_pointer as usize)
            .checked_add(nt_size)
            .and_then(|size| size.checked_add(coff.size_of_optional_header as usize))
            .ok_or(ERROR_BAD_EXE_FORMAT)?;
        let bytes = emu
            .maps
            .read_bytes_option(base, header_size)
            .ok_or(ERROR_BAD_EXE_FORMAT)?;
        let header = Header::parse(&bytes).map_err(|_| ERROR_BAD_EXE_FORMAT)?;
        if header.signature != PE_MAGIC {
            return Err(ERROR_BAD_EXE_FORMAT);
        }
        let optional = header.optional_header.ok_or(ERROR_BAD_EXE_FORMAT)?;
        let directory = optional
            .data_directories
            .get_resource_table()
            .ok_or(ERROR_RESOURCE_DATA_NOT_FOUND)?;
        let size = optional.windows_fields.size_of_image as u64;
        if directory.virtual_address == 0 || directory.size == 0 {
            return Err(ERROR_RESOURCE_DATA_NOT_FOUND);
        }
        if directory.virtual_address as u64 + directory.size as u64 > size {
            return Err(ERROR_BAD_EXE_FORMAT);
        }
        Ok(Self {
            base,
            size,
            directory: base
                .checked_add(directory.virtual_address as u64)
                .ok_or(ERROR_BAD_EXE_FORMAT)?,
            directory_size: directory.size as usize,
        })
    }

    fn lookup(
        &self,
        emu: &Emu,
        kind: ResourceKey<'_>,
        name: ResourceKey<'_>,
        language: u16,
    ) -> Result<Resource, u64> {
        let bytes = emu
            .maps
            .read_bytes_option(self.directory, self.directory_size)
            .ok_or(ERROR_RESOURCE_DATA_NOT_FOUND)?;
        lookup(&bytes, kind, name, language)
    }

    fn data(&self, emu: &Emu, resource: u64) -> Result<ImageResourceDataEntry32, u64> {
        let offset = resource
            .checked_sub(self.directory)
            .ok_or(ERROR_INVALID_HANDLE)?;
        if offset >= self.directory_size as u64 || !offset.is_multiple_of(size_of::<u32>() as u64) {
            return Err(ERROR_INVALID_HANDLE);
        }
        let bytes = emu
            .maps
            .read_bytes_option(self.directory, self.directory_size)
            .ok_or(ERROR_RESOURCE_DATA_NOT_FOUND)?;
        let data = ResourceDirectory::new(&bytes)
            .data(offset as u32)
            .map_err(resource_error)?;
        self.payload(emu, &data)?;
        Ok(data)
    }

    fn payload(&self, emu: &Emu, data: &ImageResourceDataEntry32) -> Result<u64, u64> {
        if data.offset_to_data as u64 + data.size as u64 > self.size {
            return Err(ERROR_BAD_EXE_FORMAT);
        }
        let address = self
            .base
            .checked_add(data.offset_to_data as u64)
            .ok_or(ERROR_BAD_EXE_FORMAT)?;
        if data.size != 0
            && emu
                .maps
                .read_bytes_option(address, data.size as usize)
                .is_none()
        {
            return Err(ERROR_RESOURCE_DATA_NOT_FOUND);
        }
        Ok(address)
    }
}

fn lookup(
    bytes: &[u8],
    kind: ResourceKey<'_>,
    name: ResourceKey<'_>,
    language: u16,
) -> Result<Resource, u64> {
    let directory = ResourceDirectory::new(bytes);
    if language != 0 {
        directory.find(kind, name, Some(language))
    } else {
        // The modeled user language is English. No host language/MUI lookup occurs.
        let mut found = directory.find(kind, name, Some(0));
        for language in [Some(EN_US_LOCALE as u16), None] {
            if matches!(found, Err(ResourceError::LanguageNotFound)) {
                found = directory.find(kind, name, language);
            }
        }
        found
    }
    .map_err(resource_error)
}

/// Read exactly one file-backed range. The caller supplies the emulator's allocation limit.
fn read_file_range(
    file: &mut FileHandle,
    length: u64,
    offset: u64,
    size: usize,
    limit: u64,
) -> Result<Vec<u8>, u64> {
    if offset
        .checked_add(size as u64)
        .is_none_or(|end| end > length)
    {
        return Err(ERROR_BAD_EXE_FORMAT);
    }
    if size as u64 > limit {
        return Err(ERROR_NOT_ENOUGH_MEMORY);
    }
    file.seek(SeekFrom::Start(offset))
        .map_err(|_| ERROR_READ_FAULT)?;
    let mut bytes = vec![0; size];
    let mut rest = bytes.as_mut_slice();
    while !rest.is_empty() {
        let read = file.read(rest).map_err(|_| ERROR_READ_FAULT)?;
        if read == 0 {
            return Err(ERROR_READ_FAULT);
        }
        rest = &mut rest[read..];
    }
    Ok(bytes)
}

fn file_offset(
    sections: &[SectionTable],
    header_size: u32,
    rva: u32,
    size: u32,
) -> Result<u64, u64> {
    let start = u64::from(rva);
    let end = start + u64::from(size);
    if end <= u64::from(header_size) {
        return Ok(start);
    }
    sections
        .iter()
        .find_map(|section| {
            let offset = start.checked_sub(u64::from(section.virtual_address))?;
            (offset + u64::from(size) <= u64::from(section.size_of_raw_data))
                .then(|| u64::from(section.pointer_to_raw_data) + offset)
        })
        .ok_or(ERROR_BAD_EXE_FORMAT)
}

pub(crate) fn file_payload(
    file: &mut FileHandle,
    kind: u32,
    name: u32,
    limit: u64,
) -> Result<Vec<u8>, u64> {
    let length = file.length().map_err(|_| ERROR_READ_FAULT)?;
    let dos = read_file_range(
        file,
        length,
        0,
        PE_POINTER_OFFSET as usize + size_of::<u32>(),
        limit,
    )?;
    if !dos.starts_with(&DOS_MAGIC.to_le_bytes()) {
        return Err(ERROR_BAD_EXE_FORMAT);
    }
    let pointer = u32::from_le_bytes(
        dos[PE_POINTER_OFFSET as usize..]
            .try_into()
            .map_err(|_| ERROR_BAD_EXE_FORMAT)?,
    ) as usize;
    let nt_size = SIZEOF_PE_MAGIC + SIZEOF_COFF_HEADER;
    let nt = read_file_range(file, length, pointer as u64, nt_size, limit)?;
    if !nt.starts_with(&PE_MAGIC.to_le_bytes()) {
        return Err(ERROR_BAD_EXE_FORMAT);
    }
    let mut offset = SIZEOF_PE_MAGIC;
    let coff = CoffHeader::parse(&nt, &mut offset).map_err(|_| ERROR_BAD_EXE_FORMAT)?;
    let mut section_offset = pointer
        .checked_add(nt_size + coff.size_of_optional_header as usize)
        .ok_or(ERROR_BAD_EXE_FORMAT)?;
    let header_length = section_offset
        .checked_add(coff.number_of_sections as usize * SIZEOF_SECTION_TABLE)
        .ok_or(ERROR_BAD_EXE_FORMAT)?;
    let headers = read_file_range(file, length, 0, header_length, limit)?;
    let header = Header::parse(&headers).map_err(|_| ERROR_BAD_EXE_FORMAT)?;
    let optional = header.optional_header.ok_or(ERROR_BAD_EXE_FORMAT)?;
    let sections = coff
        .sections(&headers, &mut section_offset)
        .map_err(|_| ERROR_BAD_EXE_FORMAT)?;
    let directory = optional
        .data_directories
        .get_resource_table()
        .filter(|dir| dir.virtual_address != 0 && dir.size != 0)
        .ok_or(ERROR_RESOURCE_DATA_NOT_FOUND)?;
    let header_size = optional.windows_fields.size_of_headers;
    let image_size = u64::from(optional.windows_fields.size_of_image);
    if u64::from(directory.virtual_address) + u64::from(directory.size) > image_size {
        return Err(ERROR_BAD_EXE_FORMAT);
    }
    let directory_offset = file_offset(
        &sections,
        header_size,
        directory.virtual_address,
        directory.size,
    )?;
    let directory_bytes = read_file_range(
        file,
        length,
        directory_offset,
        directory.size as usize,
        limit,
    )?;
    let entry = lookup(
        &directory_bytes,
        ResourceKey::Id(kind),
        ResourceKey::Id(name),
        0,
    )?
    .data;
    if u64::from(entry.offset_to_data) + u64::from(entry.size) > image_size {
        return Err(ERROR_BAD_EXE_FORMAT);
    }
    let start = file_offset(&sections, header_size, entry.offset_to_data, entry.size)?;
    read_file_range(file, length, start, entry.size as usize, limit)
}

pub(crate) fn numeric_payload(
    emu: &Emu,
    module: u64,
    kind: u32,
    name: u32,
) -> Result<(u64, u32), u64> {
    let image = ResourceImage::load(emu, module)?;
    let entry = image.lookup(emu, ResourceKey::Id(kind), ResourceKey::Id(name), 0)?;
    Ok((image.payload(emu, &entry.data)?, entry.data.size))
}

fn resource_error(error: ResourceError) -> u64 {
    match error {
        ResourceError::Malformed => ERROR_BAD_EXE_FORMAT,
        ResourceError::TypeNotFound => ERROR_RESOURCE_TYPE_NOT_FOUND,
        ResourceError::NameNotFound => ERROR_RESOURCE_NAME_NOT_FOUND,
        ResourceError::LanguageNotFound => ERROR_RESOURCE_LANG_NOT_FOUND,
    }
}

enum ResourceName {
    Id(u32),
    Name(String),
}

impl ResourceName {
    fn read(emu: &Emu, value: u64, wide: bool) -> Result<Self, u64> {
        if value <= u16::MAX as u64 {
            return Ok(Self::Id(value as u32));
        }
        let name = strings::read_text_checked(emu, value, wide).ok_or(ERROR_NOACCESS)?;
        if let Some(id) = name.strip_prefix('#') {
            return id
                .parse::<u16>()
                .map(|id| Self::Id(id as u32))
                .map_err(|_| ERROR_INVALID_PARAMETER);
        }
        // Windows uppercases a requested resource name before matching its stored key.
        Ok(Self::Name(name.to_uppercase()))
    }

    fn key(&self) -> ResourceKey<'_> {
        match self {
            Self::Id(id) => ResourceKey::Id(*id),
            Self::Name(name) => ResourceKey::Name(name),
        }
    }
}

fn find(emu: &mut Emu, api: &str, wide: bool, extended: bool) {
    let call = WinApi(Stdcall);
    let module = api_try!(emu, call.arg(emu, 0));
    let name = api_try!(emu, call.arg(emu, if extended { 2 } else { 1 }));
    let kind = api_try!(emu, call.arg(emu, if extended { 1 } else { 2 }));
    let language = if extended {
        api_try!(emu, call.arg_u16(emu, 3))
    } else {
        0
    };
    log_red!(
        emu,
        "{api} module=0x{module:x} type=0x{kind:x} name=0x{name:x} language=0x{language:x}"
    );
    let result = (|| {
        let name = ResourceName::read(emu, name, wide)?;
        let kind = ResourceName::read(emu, kind, wide)?;
        let image = ResourceImage::load(emu, module)?;
        let resource = image.lookup(emu, kind.key(), name.key(), language)?;
        image.payload(emu, &resource.data)?;
        image
            .directory
            .checked_add(resource.directory_offset as u64)
            .ok_or(ERROR_BAD_EXE_FORMAT)
    })();
    let result = match result {
        Ok(handle) => handle,
        Err(error) => {
            emu.set_last_error(error);
            0
        }
    };
    call.set_return(emu, result);
    call.cleanup_args(emu, if extended { 4 } else { 3 });
}

fn load(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let module = api_try!(emu, call.arg(emu, 0));
    let resource = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "{api} module=0x{module:x} resource=0x{resource:x}");
    let result = ResourceImage::load(emu, module).and_then(|image| {
        image
            .data(emu, resource)
            .and_then(|data| image.payload(emu, &data))
    });
    let result = match result {
        Ok(address) => address,
        Err(error) => {
            emu.set_last_error(error);
            0
        }
    };
    call.set_return(emu, result);
    call.cleanup_args(emu, 2);
}

fn size(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let module = api_try!(emu, call.arg(emu, 0));
    let resource = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "{api} module=0x{module:x} resource=0x{resource:x}");
    let result = ResourceImage::load(emu, module).and_then(|image| image.data(emu, resource));
    let result = match result {
        Ok(data) => data.size as u64,
        Err(error) => {
            emu.set_last_error(error);
            0
        }
    };
    call.set_return(emu, result);
    call.cleanup_args(emu, 2);
}

fn lock(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let resource = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "{api} resource=0x{resource:x}");
    let address = if resource != 0 && emu.maps.read_byte(resource).is_some() {
        resource
    } else {
        0
    };
    call.set_return(emu, address);
    call.cleanup_args(emu, 1);
}

fn free(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let resource = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "{api} resource=0x{resource:x}");
    // Win32 keeps the resource in its module; this obsolete API always returns FALSE.
    call.set_return(emu, 0);
    call.cleanup_args(emu, 1);
}

pub(crate) fn dispatch(api: &str, library: &str, emu: &mut Emu) -> bool {
    let label = format!("{library}!{api}");
    dispatch_api!(api, {
        "LoadStringA" | "LoadStringW" if library == "kernelbase" => load_string(emu,library,api),
        "FindResourceA" => find(emu, &label, false, false),
        "FindResourceW" => find(emu, &label, true, false),
        "FindResourceExA" => find(emu, &label, false, true),
        "FindResourceExW" => find(emu, &label, true, true),
        "LoadResource" => load(emu, &label),
        "SizeofResource" => size(emu, &label),
        "LockResource" => lock(emu, &label),
        "FreeResource" => free(emu, &label),
    }, handled(()) => true, fallback => false)
}

pub(crate) fn load_string(emu: &mut Emu, library: &str, api: &str) {
    const STRINGS_PER_BLOCK: u32 = 16;
    let call = WinApi(Stdcall);
    let module = api_try!(emu, call.arg(emu, 0));
    let id = api_try!(emu, call.arg_u32(emu, 1)) & u32::from(u16::MAX);
    let buffer = api_try!(emu, call.arg(emu, 2));
    let capacity = api_try!(emu, call.arg_i32(emu, 3));
    let wide = api.ends_with('W');
    log_red!(
        emu,
        "{library}!{api} module={module:#x} id={id} buffer={buffer:#x} capacity={capacity}"
    );
    let result = (|| {
        if buffer == 0 || capacity < 0 || capacity == 0 && !wide {
            return Err(ERROR_INVALID_PARAMETER);
        }
        let (mut at, size) = numeric_payload(emu, module, RT_STRING, id / STRINGS_PER_BLOCK + 1)?;
        let mut remaining = u64::from(size);
        for index in 0..=id % STRINGS_PER_BLOCK {
            if remaining < 2 {
                return Err(ERROR_BAD_EXE_FORMAT);
            }
            let len = emu.maps.read_word(at).ok_or(ERROR_BAD_EXE_FORMAT)?;
            let bytes = u64::from(len) * 2;
            at += 2;
            remaining -= 2;
            if bytes > remaining {
                return Err(ERROR_BAD_EXE_FORMAT);
            }
            if index == id % STRINGS_PER_BLOCK {
                if capacity == 0 {
                    if !writable_pointer(emu, buffer) || !write_pointer(emu, buffer, at) {
                        return Err(ERROR_NOACCESS);
                    }
                    return Ok(u64::from(len));
                }
                let encoding = if wide { Encoding::Wide } else { Encoding::Ansi };
                let n = (capacity as u64 - 1).min(u64::from(len));
                let raw =
                    read_guest_bytes(emu, at, (n * 2) as usize).ok_or(ERROR_BAD_EXE_FORMAT)?;
                let units: Vec<_> = raw
                    .chunks_exact(2)
                    .map(|b| u16::from_le_bytes([b[0], b[1]]))
                    .collect();
                let out = encoding.terminated(&units);
                if !emu.maps.validate_write_range(buffer, out.len() as u64)
                    || !emu.maps.write_bytes(buffer, &out)
                {
                    return Err(ERROR_NOACCESS);
                }
                return Ok((out.len() / encoding.width() - 1) as u64);
            }
            at += bytes;
            remaining -= bytes;
        }
        Err(ERROR_BAD_EXE_FORMAT)
    })();
    let result = result.unwrap_or_else(|e| {
        emu.set_last_error(e);
        0
    });
    call.set_return(emu, result);
    call.cleanup_args(emu, 4);
}
