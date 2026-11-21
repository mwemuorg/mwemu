//! Bounded traversal of the PE resource tree, shared by PE32 and PE32+.

use crate::pe::readers::{read_u16_le, read_u32_le};
use crate::pe::structures::{
    ImageResourceDataEntry32, ImageResourceDirectory, ImageResourceDirectoryEntry,
};

const DATA_ENTRY_SIZE: usize = 4 * size_of::<u32>();

#[derive(Clone, Copy)]
pub enum ResourceKey<'a> {
    Id(u32),
    Name(&'a str),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceError {
    Malformed,
    TypeNotFound,
    NameNotFound,
    LanguageNotFound,
}

pub struct Resource {
    /// Offset of IMAGE_RESOURCE_DATA_ENTRY relative to the resource directory.
    pub directory_offset: u32,
    pub data: ImageResourceDataEntry32,
}

pub struct ResourceDirectory<'a> {
    bytes: &'a [u8],
}

impl<'a> ResourceDirectory<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    fn range(&self, offset: usize, size: usize) -> Result<&'a [u8], ResourceError> {
        let end = offset.checked_add(size).ok_or(ResourceError::Malformed)?;
        self.bytes.get(offset..end).ok_or(ResourceError::Malformed)
    }

    pub fn name(&self, offset: usize) -> Result<String, ResourceError> {
        let length = read_u16_le(self.range(offset, size_of::<u16>())?, 0) as usize;
        let data = self.range(offset + size_of::<u16>(), length * size_of::<u16>())?;
        let units: Vec<u16> = data
            .chunks_exact(2)
            .map(|word| read_u16_le(word, 0))
            .collect();
        String::from_utf16(&units).map_err(|_| ResourceError::Malformed)
    }

    fn entries(
        &self,
        offset: usize,
    ) -> Result<impl Iterator<Item = ImageResourceDirectoryEntry> + 'a, ResourceError> {
        let header = self.range(offset, ImageResourceDirectory::size())?;
        let count = read_u16_le(header, 12) as usize + read_u16_le(header, 14) as usize;
        let bytes = self.range(
            offset + header.len(),
            count * ImageResourceDirectoryEntry::size(),
        )?;
        Ok(bytes
            .chunks_exact(ImageResourceDirectoryEntry::size())
            .map(|entry| ImageResourceDirectoryEntry {
                name_or_id: read_u32_le(entry, 0),
                data_or_directory: read_u32_le(entry, 4),
            }))
    }

    fn select(
        &self,
        offset: usize,
        key: Option<ResourceKey<'_>>,
    ) -> Result<Option<ImageResourceDirectoryEntry>, ResourceError> {
        for entry in self.entries(offset)? {
            let matches = match key {
                Some(ResourceKey::Id(id)) => entry.is_id() && entry.get_name_or_id() == id,
                Some(ResourceKey::Name(name)) => {
                    entry.is_name() && self.name(entry.get_name_or_id() as usize)? == name
                }
                None => entry.is_id(),
            };
            if matches {
                return Ok(Some(entry));
            }
        }
        Ok(None)
    }

    pub fn data(&self, offset: u32) -> Result<ImageResourceDataEntry32, ResourceError> {
        let data = self.range(offset as usize, DATA_ENTRY_SIZE)?;
        Ok(ImageResourceDataEntry32 {
            offset_to_data: read_u32_le(data, 0),
            size: read_u32_le(data, 4),
            code_page: read_u32_le(data, 8),
            reserved: read_u32_le(data, 12),
        })
    }

    pub fn find(
        &self,
        type_key: ResourceKey<'_>,
        name_key: ResourceKey<'_>,
        language: Option<u16>,
    ) -> Result<Resource, ResourceError> {
        self.find_from(0, 0, type_key, name_key, language)
    }

    pub(crate) fn find_from(
        &self,
        mut offset: usize,
        level: u32,
        type_key: ResourceKey<'_>,
        name_key: ResourceKey<'_>,
        language: Option<u16>,
    ) -> Result<Resource, ResourceError> {
        if level > 2 {
            return Err(ResourceError::Malformed);
        }
        for (level, (key, missing)) in [
            (Some(type_key), ResourceError::TypeNotFound),
            (Some(name_key), ResourceError::NameNotFound),
            (
                language.map(|id| ResourceKey::Id(id as u32)),
                ResourceError::LanguageNotFound,
            ),
        ]
        .into_iter()
        .enumerate()
        .skip(level as usize)
        {
            let entry = self.select(offset, key)?.ok_or(missing)?;
            if entry.is_directory() != (level < 2) {
                return Err(ResourceError::Malformed);
            }
            offset = entry.get_offset() as usize;
        }
        Ok(Resource {
            directory_offset: offset as u32,
            data: self.data(offset as u32)?,
        })
    }
}
