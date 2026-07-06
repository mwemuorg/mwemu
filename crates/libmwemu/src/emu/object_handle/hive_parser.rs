use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HiveError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid hive signature")]
    InvalidSignature,
    #[error("File too small")]
    FileTooSmall,
    #[error("Invalid block type")]
    InvalidBlockType,
    #[error("Key not found")]
    KeyNotFound,
    #[error("Value not found")]
    ValueNotFound,
    #[error("Invalid value type")]
    InvalidValueType,
    #[error("Name buffer overflow")]
    NameBufferOverflow,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Offsets {
    block_size: i32,
    block_type: [u8; 2],
    count: i16,
    first: i32,
    hash: i32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct KeyBlock {
    block_size: i32,
    block_type: [u8; 2],
    pub(crate) subkey_count: i32,
    pub(crate) subkeys_offset: i32,
    value_count: i32,
    offsets_offset: i32,
    name_len: i16,
    name: [u8; 255],
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ValueBlock {
    block_size: i32,
    block_type: [u8; 2],
    name_len: i16,
    size: i32,
    data_offset: i32,
    value_type: RegType,
    name: [u8; 255],
}

#[derive(Debug)]
pub enum RegistryValue {
    String(String),
    Dword(u32),
    Binary(Vec<u8>),
    MultiString(Vec<String>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum RegType {
    None = 0,                      // REG_NONE
    Sz = 1,                        // REG_SZ - Unicode null-terminated string
    ExpandSz = 2,                  // REG_EXPAND_SZ - String with env vars (%PATH%)
    Binary = 3,                    // REG_BINARY - Binary data
    DWord = 4,                     // REG_DWORD / REG_DWORD_LITTLE_ENDIAN - 32-bit LE integer
    DWordBigEndian = 5,            // REG_DWORD_BIG_ENDIAN - 32-bit BE integer
    Link = 6,                      // REG_LINK - Symbolic link (Unicode string)
    MultiSz = 7,      // REG_MULTI_SZ - Multiple Unicode strings (double-null terminated)
    ResourceList = 8, // REG_RESOURCE_LIST - Plug and Play resource list
    FullResourceDescriptor = 9, // REG_FULL_RESOURCE_DESCRIPTOR - Full resource descriptor
    ResourceRequirementsList = 10, // REG_RESOURCE_REQUIREMENTS_LIST
    QWord = 11,       // REG_QWORD / REG_QWORD_LITTLE_ENDIAN - 64-bit LE integer
}

impl TryFrom<u32> for RegType {
    type Error = RegTypeError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Sz),
            2 => Ok(Self::ExpandSz),
            3 => Ok(Self::Binary),
            4 => Ok(Self::DWord),
            5 => Ok(Self::DWordBigEndian),
            6 => Ok(Self::Link),
            7 => Ok(Self::MultiSz),
            8 => Ok(Self::ResourceList),
            9 => Ok(Self::FullResourceDescriptor),
            10 => Ok(Self::ResourceRequirementsList),
            11 => Ok(Self::QWord),
            _ => Err(RegTypeError(value)),
        }
    }
}

/// Error returned when an invalid registry type value is encountered
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegTypeError(pub u32);

impl fmt::Display for RegTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid registry type value: {}", self.0)
    }
}

pub struct HiveKey<'a, R: Read + Seek> {
    pub(crate) key_block: KeyBlock,
    pub(crate) base_offset: u64,
    pub(crate) reader: &'a mut R,
}

#[derive(Debug, Clone)]
struct HiveSubpath {
    path: String,
    key_offset: u64,
}

#[derive(Debug, Clone)]
struct HiveCache {
    main_key_offset: u64,
    subpaths: Vec<HiveSubpath>,
}
pub struct HiveParser<R: Read + Seek> {
    pub(crate) reader: R, // Own the file instead of referencing it
    pub(crate) base_offset: u64,
    subkey_cache: HashMap<String, HiveCache>,
}

impl Offsets {
    pub(crate) fn read_from_file<R: Read + Seek>(
        reader: &mut R,
        offset: u64,
    ) -> Result<Self, HiveError> {
        reader.seek(SeekFrom::Start(offset))?;

        let block_size = reader.read_i32::<LittleEndian>()?;
        let block_type = [reader.read_u8()?, reader.read_u8()?];
        let count = reader.read_i16::<LittleEndian>()?;
        let first = reader.read_i32::<LittleEndian>()?;
        let hash = reader.read_i32::<LittleEndian>()?;

        Ok(Self {
            block_size,
            block_type,
            count,
            first,
            hash,
        })
    }
}

impl KeyBlock {
    pub(crate) fn read_from_reader<R: Read + Seek>(
        reader: &mut R,
        offset: u64,
    ) -> Result<Self, HiveError> {
        reader.seek(SeekFrom::Start(offset))?;

        let block_size = reader.read_i32::<LittleEndian>()?;
        let block_type = [reader.read_u8()?, reader.read_u8()?];

        // Skip dummy data (18 bytes)
        let mut dummy = [0u8; 18];
        reader.read_exact(&mut dummy)?;

        let subkey_count = reader.read_i32::<LittleEndian>()?;

        // Skip dummy data (4 bytes)
        let mut dummy = [0u8; 4];
        reader.read_exact(&mut dummy)?;
        let subkeys_offset = reader.read_i32::<LittleEndian>()?;

        // Skip dummy data (4 bytes)
        let mut dummy = [0u8; 4];
        reader.read_exact(&mut dummy)?;

        let value_count = reader.read_i32::<LittleEndian>()?;
        let offsets_offset = reader.read_i32::<LittleEndian>()?;

        // Skip dummy data (28 bytes)
        let mut dummy = [0u8; 28];
        reader.read_exact(&mut dummy)?;

        let name_len = reader.read_i16::<LittleEndian>()?;

        // Skip dummy data (2 bytes)
        let mut dummy = [0u8; 2];
        reader.read_exact(&mut dummy)?;

        let mut name = [0u8; 255];
        reader.read_exact(&mut name)?;

        Ok(Self {
            block_size,
            block_type,
            subkey_count,
            subkeys_offset,
            value_count,
            offsets_offset,
            name_len,
            name,
        })
    }

    pub(crate) fn get_name(&self) -> Result<String, HiveError> {
        if self.name_len <= 0 || self.name_len as usize > self.name.len() {
            return Err(HiveError::NameBufferOverflow);
        }

        let name_str = String::from_utf8_lossy(&self.name[..self.name_len as usize]).into_owned();
        Ok(name_str)
    }
}

impl ValueBlock {
    fn read_from_reader<R: Read + Seek>(reader: &mut R, offset: u64) -> Result<Self, HiveError> {
        reader.seek(SeekFrom::Start(offset))?;

        let block_size = reader.read_i32::<LittleEndian>()?;
        let block_type = [reader.read_u8()?, reader.read_u8()?];
        let name_len = reader.read_i16::<LittleEndian>()?;
        let size = reader.read_i32::<LittleEndian>()?;
        let data_offset = reader.read_i32::<LittleEndian>()?;
        let value_type = (reader.read_i32::<LittleEndian>()? as u32)
            .try_into()
            .unwrap();

        // Skip flags and dummy (4 bytes)
        let mut dummy = [0u8; 4];
        reader.read_exact(&mut dummy)?;

        let mut name = [0u8; 255];
        reader.read_exact(&mut name)?;

        Ok(Self {
            block_size,
            block_type,
            name_len,
            size,
            data_offset,
            value_type,
            name,
        })
    }

    fn get_name(&self) -> Result<String, HiveError> {
        if self.name_len <= 0 || self.name_len as usize > self.name.len() {
            return Err(HiveError::NameBufferOverflow);
        }

        let name_str = String::from_utf8_lossy(&self.name[..self.name_len as usize]).into_owned();
        Ok(name_str)
    }
}

impl<'a, R: Read + Seek> HiveKey<'a, R> {
    pub fn new(key_block: KeyBlock, base_offset: u64, reader: &'a mut R) -> Self {
        Self {
            key_block,
            base_offset,
            reader,
        }
    }

    pub fn subkeys_list(&mut self) -> Result<Vec<String>, HiveError> {
        let offsets_offset = self.base_offset + self.key_block.subkeys_offset as u64;
        let offsets = Offsets::read_from_file(self.reader, offsets_offset)?;

        // Check block type ('f' or 'h')
        if offsets.block_type[1] != b'f' && offsets.block_type[1] != b'h' {
            return Err(HiveError::InvalidBlockType);
        }

        let mut result = Vec::with_capacity(self.key_block.subkey_count as usize);

        for i in 0..self.key_block.subkey_count {
            // Read the offset from the offsets table
            let offset_entry_offset = offsets_offset + 16 + (i as u64 * 8); // 16 = size of Offsets struct before 'first'
            self.reader.seek(SeekFrom::Start(offset_entry_offset))?;
            let subkey_offset = self.reader.read_i32::<LittleEndian>()? as u64;

            if subkey_offset == 0 {
                continue;
            }

            let subkey_abs_offset = self.base_offset + subkey_offset;
            let subkey = KeyBlock::read_from_reader(self.reader, subkey_abs_offset)?;

            let name = subkey.get_name()?;
            result.push(name);
        }

        Ok(result)
    }

    pub fn keys_list(&mut self) -> Result<Vec<String>, HiveError> {
        if self.key_block.value_count == 0 {
            return Ok(Vec::new());
        }

        let mut result = Vec::with_capacity(self.key_block.value_count as usize);

        // Read value offsets
        let offsets_base = self.base_offset + self.key_block.offsets_offset as u64 + 4;

        for i in 0..self.key_block.value_count {
            let offset_entry_offset = offsets_base + (i as u64 * 4);
            self.reader.seek(SeekFrom::Start(offset_entry_offset))?;
            let value_offset = self.reader.read_i32::<LittleEndian>()? as u64;

            if value_offset == 0 {
                continue;
            }

            let value_abs_offset = self.base_offset + value_offset;
            let value = ValueBlock::read_from_reader(self.reader, value_abs_offset)?;

            let name = value.get_name()?;
            result.push(name);
        }

        Ok(result)
    }

    pub fn get_key_value<T>(&mut self, name: &str) -> Result<Option<T>, HiveError>
    where
        T: TryFrom<RegistryValue, Error = HiveError>,
    {
        if self.key_block.value_count == 0 {
            return Ok(None);
        }

        let offsets_base = self.base_offset + self.key_block.offsets_offset as u64 + 4;

        for i in 0..self.key_block.value_count {
            let offset_entry_offset = offsets_base + (i as u64 * 4);
            self.reader.seek(SeekFrom::Start(offset_entry_offset))?;
            let value_offset = self.reader.read_i32::<LittleEndian>()? as u64;

            if value_offset == 0 {
                continue;
            }

            let value_abs_offset = self.base_offset + value_offset;
            let value = ValueBlock::read_from_reader(self.reader, value_abs_offset)?;

            let value_name = value.get_name()?;
            if value_name != name {
                continue;
            }

            // Read the actual value data
            let registry_value = self.read_value_data(&value)?;

            return match T::try_from(registry_value) {
                Ok(val) => Ok(Some(val)),
                Err(_) => Err(HiveError::InvalidValueType),
            };
        }

        Ok(None)
    }

    pub fn get_key_value_wrap(&mut self, name: &str) -> Result<Option<RegistryValue>, HiveError> {
        if self.key_block.value_count == 0 {
            return Ok(None);
        }

        let offsets_base = self.base_offset + self.key_block.offsets_offset as u64 + 4;

        for i in 0..self.key_block.value_count {
            let offset_entry_offset = offsets_base + (i as u64 * 4);
            self.reader.seek(SeekFrom::Start(offset_entry_offset))?;
            let value_offset = self.reader.read_i32::<LittleEndian>()? as u64;

            if value_offset == 0 {
                continue;
            }

            let value_abs_offset = self.base_offset + value_offset;
            let value = ValueBlock::read_from_reader(self.reader, value_abs_offset)?;

            let value_name = value.get_name()?;
            if value_name != name {
                continue;
            }

            // Read the actual value data
            let registry_value = self.read_value_data(&value)?;

            return Ok(Some(registry_value));
        }

        Ok(None)
    }

    fn read_value_data(&mut self, value: &ValueBlock) -> Result<RegistryValue, HiveError> {
        let data_size = value.size & 0xFFFF;
        let is_inline = (value.size & (1 << 31)) != 0;

        if is_inline {
            // Inline data - stored directly in the value block
            let mut buffer = vec![0u8; data_size as usize];
            // The inline data is stored in the data_offset field itself
            let inline_data = value.data_offset.to_le_bytes();
            let copy_len = std::cmp::min(inline_data.len(), buffer.len());
            buffer[..copy_len].copy_from_slice(&inline_data[..copy_len]);
            return Ok(RegistryValue::Binary(buffer));
        }

        // External data - stored at the data offset
        let data_offset = self.base_offset + value.data_offset as u64 + 4;
        self.reader.seek(SeekFrom::Start(data_offset))?;

        match value.value_type {
            RegType::Sz | RegType::ExpandSz => {
                // REG_SZ, REG_EXPAND_SZ
                let mut buffer = vec![0u8; data_size as usize];
                self.reader.read_exact(&mut buffer)?;
                let text = String::from_utf8_lossy(&buffer).into_owned();
                Ok(RegistryValue::String(text))
            }
            RegType::Binary => {
                // REG_BINARY
                let mut buffer = vec![0u8; data_size as usize];
                self.reader.read_exact(&mut buffer)?;
                Ok(RegistryValue::Binary(buffer))
            }
            RegType::DWord => {
                // REG_DWORD
                let dword = self.reader.read_u32::<LittleEndian>()?;
                Ok(RegistryValue::Dword(dword))
            }
            RegType::MultiSz => {
                // REG_MULTI_SZ
                let mut buffer = vec![0u8; data_size as usize];
                self.reader.read_exact(&mut buffer)?;

                let mut strings = Vec::new();
                let mut current = String::new();

                for &byte in &buffer {
                    if byte == 0 {
                        if !current.is_empty() {
                            strings.push(current.clone());
                            current.clear();
                        }
                    } else {
                        current.push(byte as char);
                    }
                }

                if !current.is_empty() {
                    strings.push(current);
                }

                Ok(RegistryValue::MultiString(strings))
            }
            _ => Err(HiveError::InvalidValueType),
        }
    }
}

impl<R: Read + Seek> HiveParser<R> {
    pub fn from_reader(mut reader: R) -> Result<Self, HiveError> {
        // Check signature
        let mut signature = [0u8; 4];
        reader.read_exact(&mut signature)?;
        if &signature != b"regf" {
            return Err(HiveError::InvalidSignature);
        }

        // Base offset is 0x1000 (4096) for the hive data
        let base_offset = 0x1000;

        // Main key block is at 0x1020 from the start of the file
        let main_key_offset = base_offset + 0x20;
        let main_key = KeyBlock::read_from_reader(&mut reader, main_key_offset)?;

        let mut parser = Self {
            reader,
            base_offset,
            subkey_cache: HashMap::new(),
        };

        parser.build_cache("", main_key_offset)?;
        Ok(parser)
    }
}

impl HiveParser<File> {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, HiveError> {
        let file = File::open(path)?;
        Self::from_reader(file)
    }
}

impl<R: Read + Seek> HiveParser<R> {
    fn build_cache(&mut self, current_path: &str, key_offset: u64) -> Result<(), HiveError> {
        let key = KeyBlock::read_from_reader(&mut self.reader, key_offset)?;

        if key.subkey_count == 0 {
            return Ok(());
        }

        let offsets_offset = self.base_offset + key.subkeys_offset as u64;
        let offsets = Offsets::read_from_file(&mut self.reader, offsets_offset)?;

        if offsets.block_type[1] != b'f' && offsets.block_type[1] != b'h' {
            return Err(HiveError::InvalidBlockType);
        }

        let mut subpaths = Vec::new();

        for i in 0..key.subkey_count {
            let offset_entry_offset = offsets_offset + 16 + (i as u64 * 8);
            self.reader.seek(SeekFrom::Start(offset_entry_offset))?;
            let subkey_offset = self.reader.read_i32::<LittleEndian>()? as u64;

            if subkey_offset == 0 {
                continue;
            }

            let subkey_abs_offset = self.base_offset + subkey_offset;
            let subkey = KeyBlock::read_from_reader(&mut self.reader, subkey_abs_offset)?;

            let subkey_name = subkey.get_name()?;
            let full_path = if current_path.is_empty() {
                subkey_name.clone()
            } else {
                format!("{}/{}", current_path, subkey_name)
            };

            // For first-level keys, create cache entries
            if current_path.is_empty() {
                self.subkey_cache
                    .entry(subkey_name.clone())
                    .or_insert_with(|| HiveCache {
                        main_key_offset: subkey_abs_offset,
                        subpaths: Vec::new(),
                    });
            }

            subpaths.push(HiveSubpath {
                path: full_path.clone(),
                key_offset: subkey_abs_offset,
            });

            // Recursively process subkeys
            if subkey.subkey_count > 0 {
                self.build_cache(&full_path, subkey_abs_offset)?;
            }
        }

        // Add subpaths to parent cache if this is not the root
        if !current_path.is_empty() {
            if let Some(cache) = self.subkey_cache.get_mut(current_path) {
                cache.subpaths.extend(subpaths);
            }
        }

        Ok(())
    }

    pub fn get_subkey(
        &mut self,
        key_name: &str,
        path: &str,
    ) -> Result<Option<HiveKey<'_, R>>, HiveError> {
        if let Some(cache) = self.subkey_cache.get(key_name) {
            for subpath in &cache.subpaths {
                if subpath.path == path {
                    let key_block =
                        KeyBlock::read_from_reader(&mut self.reader, subpath.key_offset)?;
                    return Ok(Some(HiveKey::new(
                        key_block,
                        self.base_offset,
                        &mut self.reader,
                    )));
                }
            }
        }
        Ok(None)
    }

    pub fn success(&self) -> bool {
        !self.subkey_cache.is_empty()
    }
}

// Conversion traits for convenience
impl TryFrom<RegistryValue> for String {
    type Error = HiveError;

    fn try_from(value: RegistryValue) -> Result<Self, Self::Error> {
        match value {
            RegistryValue::String(s) => Ok(s),
            _ => Err(HiveError::InvalidValueType),
        }
    }
}

impl TryFrom<RegistryValue> for u32 {
    type Error = HiveError;

    fn try_from(value: RegistryValue) -> Result<Self, Self::Error> {
        match value {
            RegistryValue::Dword(d) => Ok(d),
            _ => Err(HiveError::InvalidValueType),
        }
    }
}

impl TryFrom<RegistryValue> for Vec<u8> {
    type Error = HiveError;

    fn try_from(value: RegistryValue) -> Result<Self, Self::Error> {
        match value {
            RegistryValue::Binary(b) => Ok(b),
            _ => Err(HiveError::InvalidValueType),
        }
    }
}

impl TryFrom<RegistryValue> for Vec<String> {
    type Error = HiveError;

    fn try_from(value: RegistryValue) -> Result<Self, Self::Error> {
        match value {
            RegistryValue::MultiString(s) => Ok(s),
            _ => Err(HiveError::InvalidValueType),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;
    use std::io::Cursor;
    #[test]
    fn test_key_block_get_name() {
        let mut name_buffer = [0u8; 255];
        let test_name = "TestKeyName";
        let name_bytes = test_name.as_bytes();
        name_buffer[..name_bytes.len()].copy_from_slice(name_bytes);

        let key_block = KeyBlock {
            block_size: 4096,
            block_type: [b'n', b'k'],
            subkey_count: 0,
            subkeys_offset: 0,
            value_count: 0,
            offsets_offset: 0,
            name_len: test_name.len() as i16,
            name: name_buffer,
        };

        let result = key_block.get_name();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_name);
    }

    #[test]
    fn test_key_block_get_name_overflow() {
        let name_buffer = [0u8; 255];

        let key_block = KeyBlock {
            block_size: 4096,
            block_type: [b'n', b'k'],
            subkey_count: 0,
            subkeys_offset: 0,
            value_count: 0,
            offsets_offset: 0,
            name_len: 300, // Greater than buffer size
            name: name_buffer,
        };

        let result = key_block.get_name();
        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            HiveError::NameBufferOverflow
        ));
    }

    #[test]
    fn test_registry_value_conversions() {
        // Test String conversion
        let string_val = RegistryValue::String("test".to_string());
        let converted_string: String = string_val.try_into().unwrap();
        assert_eq!(converted_string, "test");

        // Test Dword conversion
        let dword_val = RegistryValue::Dword(12345);
        let converted_dword: u32 = dword_val.try_into().unwrap();
        assert_eq!(converted_dword, 12345);

        // Test Binary conversion
        let binary_val = RegistryValue::Binary(vec![1, 2, 3, 4]);
        let converted_binary: Vec<u8> = binary_val.try_into().unwrap();
        assert_eq!(converted_binary, vec![1, 2, 3, 4]);

        // Test MultiString conversion
        let multi_val = RegistryValue::MultiString(vec!["one".to_string(), "two".to_string()]);
        let converted_multi: Vec<String> = multi_val.try_into().unwrap();
        assert_eq!(converted_multi, vec!["one".to_string(), "two".to_string()]);

        // Test invalid conversions
        let string_val = RegistryValue::String("test".to_string());
        let result: Result<u32, _> = string_val.try_into();
        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), HiveError::InvalidValueType));
    }

    #[test]
    fn test_value_block_get_name() {
        let mut name_buffer = [0u8; 255];
        let test_name = "TestValueName";
        let name_bytes = test_name.as_bytes();
        name_buffer[..name_bytes.len()].copy_from_slice(name_bytes);

        let value_block = ValueBlock {
            block_size: 512,
            block_type: [b'v', b'k'],
            name_len: test_name.len() as i16,
            size: 4,
            data_offset: 1024,
            value_type: 3.try_into().unwrap(), // REG_BINARY
            name: name_buffer,
        };

        let result = value_block.get_name();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_name);
    }

    #[test]
    fn test_hive_error_display() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        let hive_error: HiveError = io_error.into();
        assert!(hive_error.to_string().contains("IO error"));

        assert_eq!(
            HiveError::InvalidSignature.to_string(),
            "Invalid hive signature"
        );
        assert_eq!(HiveError::FileTooSmall.to_string(), "File too small");
        assert_eq!(
            HiveError::InvalidBlockType.to_string(),
            "Invalid block type"
        );
        assert_eq!(HiveError::KeyNotFound.to_string(), "Key not found");
        assert_eq!(HiveError::ValueNotFound.to_string(), "Value not found");
        assert_eq!(
            HiveError::InvalidValueType.to_string(),
            "Invalid value type"
        );
        assert_eq!(
            HiveError::NameBufferOverflow.to_string(),
            "Name buffer overflow"
        );
    }

    // Test with minimal valid hive structure
    fn create_minimal_hive_data() -> Vec<u8> {
        let mut data = Vec::new();

        // 1. Header (4096 bytes)
        // Signature "regf"
        data.extend_from_slice(b"regf");
        // Fill the rest of header with zeros
        data.resize(4096, 0);

        // 2. Root key block at offset 0x1020 (4128)
        data.resize(4128, 0);

        // Write root key block structure
        let root_key_offset = 4128;

        // Block size (4096)
        data.extend_from_slice(&4096i32.to_le_bytes());
        // Block type "nk" (0x6B, 0x6E)
        data.extend_from_slice(&[0x6B, 0x6E]);
        // Skip 18 bytes dummy
        data.extend_from_slice(&[0u8; 18]);
        // Subkey count (0)
        data.extend_from_slice(&0i32.to_le_bytes());
        // Skip 4 bytes dummy
        data.extend_from_slice(&[0u8; 4]);
        // Subkeys offset (0)
        data.extend_from_slice(&0i32.to_le_bytes());
        // Skip 4 bytes dummy
        data.extend_from_slice(&[0u8; 4]);
        // Value count (1)
        data.extend_from_slice(&1i32.to_le_bytes());
        // Value offsets offset
        data.extend_from_slice(&100i32.to_le_bytes());
        // Skip 28 bytes dummy
        data.extend_from_slice(&[0u8; 28]);
        // Name length (0 for root)
        data.extend_from_slice(&0i16.to_le_bytes());
        // Skip 2 bytes dummy
        data.extend_from_slice(&[0u8; 2]);
        // Name (empty)
        data.extend_from_slice(&[0u8; 255]);

        // Pad to 4096 bytes
        data.resize(root_key_offset + 4096, 0);

        // 3. Value block at offset 100 from root (4228)
        let value_offset = 4228;
        // Block size
        data.extend_from_slice(&512i32.to_le_bytes());
        // Block type "vk"
        data.extend_from_slice(&[0x6B, 0x76]);
        // Name length (11)
        data.extend_from_slice(&11i16.to_le_bytes());
        // Size (4, with inline flag set)
        data.extend_from_slice(&(4i32 | (1i32 << 31i32)).to_le_bytes());
        // Data offset (0x12345678 - will be used as inline data)
        data.extend_from_slice(&0x78563412i32.to_le_bytes()); // Little endian of 0x12345678
        // Value type (REG_DWORD = 4)
        data.extend_from_slice(&4i32.to_le_bytes());
        // Skip 4 bytes
        data.extend_from_slice(&[0u8; 4]);
        // Name "TestValue"
        let name = b"TestValue\0\0";
        data.extend_from_slice(name);
        data.resize(data.len() + 255 - name.len(), 0);

        data
    }

    #[test]
    fn test_offsets_read() {
        let mut data = Vec::new();
        data.extend_from_slice(&4096i32.to_le_bytes()); // block_size
        data.extend_from_slice(&[b'l', b'f']); // block_type
        data.extend_from_slice(&10i16.to_le_bytes()); // count
        data.extend_from_slice(&100i32.to_le_bytes()); // first
        data.extend_from_slice(&200i32.to_le_bytes()); // hash

        let mut cursor = Cursor::new(data);
        let result = Offsets::read_from_file(&mut cursor, 0);

        assert!(result.is_ok());
        let offsets = result.unwrap();
        assert_eq!(offsets.block_size, 4096);
        assert_eq!(offsets.block_type, [b'l', b'f']);
        assert_eq!(offsets.count, 10);
        assert_eq!(offsets.first, 100);
        assert_eq!(offsets.hash, 200);
    }

    #[test]
    fn test_key_block_read() {
        let mut data = Vec::new();
        data.extend_from_slice(&4096i32.to_le_bytes()); // block_size
        data.extend_from_slice(&[b'n', b'k']); // block_type
        data.extend_from_slice(&[0u8; 18]); // dummy
        data.extend_from_slice(&5i32.to_le_bytes()); // subkey_count
        data.extend_from_slice(&[0u8; 4]); // dummy
        data.extend_from_slice(&1000i32.to_le_bytes()); // subkeys_offset
        data.extend_from_slice(&[0u8; 4]); // dummy
        data.extend_from_slice(&3i32.to_le_bytes()); // value_count
        data.extend_from_slice(&2000i32.to_le_bytes()); // offsets_offset
        data.extend_from_slice(&[0u8; 28]); // dummy
        data.extend_from_slice(&8i16.to_le_bytes()); // name_len
        data.extend_from_slice(&[0u8; 2]); // dummy
        data.extend_from_slice(b"TestKey\0"); // name
        data.resize(data.len() + 255 - 8, 0); // pad to 255

        let mut cursor = Cursor::new(data);
        let result = KeyBlock::read_from_reader(&mut cursor, 0);

        assert!(result.is_ok());
        let key_block = result.unwrap();
        assert_eq!(key_block.block_size, 4096);
        assert_eq!(key_block.block_type, [b'n', b'k']);
        assert_eq!(key_block.subkey_count, 5);
        assert_eq!(key_block.subkeys_offset, 1000);
        assert_eq!(key_block.value_count, 3);
        assert_eq!(key_block.offsets_offset, 2000);
        assert_eq!(key_block.name_len, 8);
        assert_eq!(key_block.get_name().unwrap().into_bytes(), b"TestKey\0");
    }

    #[test]
    fn test_value_block_read() {
        let mut data = Vec::new();
        data.extend_from_slice(&512i32.to_le_bytes()); // block_size
        data.extend_from_slice(&[b'v', b'k']); // block_type
        data.extend_from_slice(&9i16.to_le_bytes()); // name_len
        data.extend_from_slice(&4i32.to_le_bytes()); // size
        data.extend_from_slice(&1024i32.to_le_bytes()); // data_offset
        data.extend_from_slice(&4i32.to_le_bytes()); // value_type (REG_DWORD)
        data.extend_from_slice(&[0u8; 4]); // flags and dummy
        data.extend_from_slice(b"MyValue\0\0"); // name
        data.resize(data.len() + 255 - 9, 0); // pad to 255

        let mut cursor = Cursor::new(data);
        let result = ValueBlock::read_from_reader(&mut cursor, 0);

        assert!(result.is_ok());
        let value_block = result.unwrap();
        assert_eq!(value_block.block_size, 512);
        assert_eq!(value_block.block_type, [b'v', b'k']);
        assert_eq!(value_block.name_len, 9);
        assert_eq!(value_block.size, 4);
        assert_eq!(value_block.data_offset, 1024);
        assert_eq!(value_block.value_type, 4.try_into().unwrap());
        assert_eq!(value_block.get_name().unwrap().into_bytes(), b"MyValue\0\0");
    }
}
