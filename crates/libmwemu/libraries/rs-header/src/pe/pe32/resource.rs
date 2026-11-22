use crate::pe::pe32::PE32;
use crate::pe::resource::{ResourceDirectory, ResourceKey};
use crate::pe::shared::IMAGE_DIRECTORY_ENTRY_RESOURCE;
use crate::pe::structures::{ImageResourceDataEntry32, ImageResourceDirectoryEntry};

impl PE32 {
    fn resource_bytes<'a>(&self, raw: &'a [u8]) -> Option<&'a [u8]> {
        let directory = self
            .opt
            .data_directory
            .get(IMAGE_DIRECTORY_ENTRY_RESOURCE)?;
        if directory.virtual_address == 0 || directory.size == 0 {
            return None;
        }
        self.image_layout()
            .data_at(raw, directory.virtual_address)?
            .get(..directory.size as usize)
    }

    pub fn locate_resource_data_entry(
        &self,
        _raw: &[u8],
        rsrc: &[u8],
        off: usize,
        level: u32,
        type_id: Option<u32>,
        name_id: Option<u32>,
        type_name: Option<&str>,
        name: Option<&str>,
    ) -> Option<ImageResourceDataEntry32> {
        let type_key = type_id
            .map(ResourceKey::Id)
            .or_else(|| type_name.map(ResourceKey::Name))?;
        let name_key = name_id
            .map(ResourceKey::Id)
            .or_else(|| name.map(ResourceKey::Name))?;
        let data = ResourceDirectory::new(rsrc)
            .find_from(off, level, type_key, name_key, None)
            .ok()?
            .data;
        Some(data)
    }

    /// Return the payload RVA and length; neither field is a file offset or VA.
    pub fn get_resource(
        &self,
        raw: &[u8],
        type_id: Option<u32>,
        name_id: Option<u32>,
        type_name: Option<&str>,
        name: Option<&str>,
    ) -> Option<(u64, usize)> {
        let type_key = type_id
            .map(ResourceKey::Id)
            .or_else(|| type_name.map(ResourceKey::Name))?;
        let name_key = name_id
            .map(ResourceKey::Id)
            .or_else(|| name.map(ResourceKey::Name))?;
        let data = ResourceDirectory::new(self.resource_bytes(raw)?)
            .find(type_key, name_key, None)
            .ok()?
            .data;
        let layout = self.image_layout();
        let end = data.offset_to_data as u64 + data.size as u64;
        if end > layout.mapped_size() {
            return None;
        }
        if data.size != 0 {
            layout
                .data_at(raw, data.offset_to_data)?
                .get(..data.size as usize)?;
        }
        Some((data.offset_to_data as u64, data.size as usize))
    }

    pub fn get_resource_name(&self, raw: &[u8], entry: &ImageResourceDirectoryEntry) -> String {
        self.resource_bytes(raw)
            .and_then(|bytes| {
                ResourceDirectory::new(bytes)
                    .name(entry.get_name_or_id() as usize)
                    .ok()
            })
            .unwrap_or_default()
    }
}
