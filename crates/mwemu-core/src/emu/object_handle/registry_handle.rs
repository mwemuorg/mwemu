use crate::emu::object_handle::hive_parser::{
    HiveError, HiveKey, HiveParser, KeyBlock, RegistryValue,
};
use ahash::AHashMap;
use byteorder::ReadBytesExt;
use std::io::{Read, Seek};
use std::path::Path;

#[derive(Debug)]
pub struct RegistryHandle {
    registry_value: RegistryValue,
}

#[derive(Debug)]
pub struct RegisterManager {
    name: String,
    registry_handle: AHashMap<String, RegistryHandle>,
    subkeys: AHashMap<String, RegisterManager>, // Added for subkey hierarchy
}

#[derive(Debug)]
pub struct RootRegistryManager {
    level_count: u16,
    next_level_ptr: AHashMap<String, RegisterManager>, // Changed to HashMap for fast access
}

impl RootRegistryManager {
    /// Load a registry hive file and build an in-memory representation
    pub fn from_hive_file<P: AsRef<Path>>(path: P) -> Result<Self, HiveError> {
        let mut parser = HiveParser::from_file(path)?;

        // Get the root key from the parser
        let root_key_block = {
            let main_key_offset = parser.base_offset + 0x20;
            KeyBlock::read_from_reader(&mut parser.reader, main_key_offset)?
        };

        let mut root_hive_key =
            HiveKey::new(root_key_block, parser.base_offset, &mut parser.reader);

        // Build the registry tree starting from root
        let root_manager = Self::build_register_manager_from_hive_key(&mut root_hive_key)?;

        // Calculate maximum depth
        let level_count = Self::compute_max_depth(&root_manager);

        Ok(Self {
            level_count,
            next_level_ptr: root_manager.subkeys, // Root's subkeys become our first level
        })
    }

    /// Build a RegisterManager from a HiveKey (recursive)
    fn build_register_manager_from_hive_key<R: Read + Seek>(
        hive_key: &mut HiveKey<R>,
    ) -> Result<RegisterManager, HiveError> {
        let key_name = hive_key.key_block.get_name()?;

        let mut reg_mgr = RegisterManager {
            name: key_name.clone(),
            registry_handle: AHashMap::new(),
            subkeys: AHashMap::new(),
        };

        // Load all values for this key
        Self::load_key_values(hive_key, &mut reg_mgr)?;

        // Load all subkeys recursively
        Self::load_subkeys(hive_key, &mut reg_mgr)?;

        Ok(reg_mgr)
    }

    /// Load all values from a HiveKey into a RegisterManager
    fn load_key_values<R: Read + Seek>(
        hive_key: &mut HiveKey<R>,
        reg_mgr: &mut RegisterManager,
    ) -> Result<(), HiveError> {
        let value_names = hive_key.keys_list()?;

        for value_name in value_names {
            // Use the non-generic version to get RegistryValue
            if let Some(registry_value) = hive_key.get_key_value_as_registry(&value_name)? {
                reg_mgr
                    .registry_handle
                    .insert(value_name, RegistryHandle { registry_value });
            }
        }

        Ok(())
    }

    /// Load all subkeys recursively
    fn load_subkeys<R: Read + Seek>(
        hive_key: &mut HiveKey<R>,
        reg_mgr: &mut RegisterManager,
    ) -> Result<(), HiveError> {
        let subkey_names = hive_key.subkeys_list()?;

        for subkey_name in subkey_names {
            if let Some(mut subkey_hive_key) = hive_key.get_subkey_by_name(&subkey_name)? {
                let subkey_reg_mgr =
                    Self::build_register_manager_from_hive_key(&mut subkey_hive_key)?;
                reg_mgr.subkeys.insert(subkey_name, subkey_reg_mgr);
            }
        }

        Ok(())
    }

    /// Compute maximum depth of the registry tree
    fn compute_max_depth(reg_mgr: &RegisterManager) -> u16 {
        let mut max_depth = 0;

        for (_, subkey) in &reg_mgr.subkeys {
            let depth = Self::compute_max_depth(subkey) + 1;
            if depth > max_depth {
                max_depth = depth;
            }
        }

        max_depth
    }

    /// Get a value by path (e.g., "SOFTWARE\\Microsoft\\Windows\\CurrentVersion")
    pub fn get_value_by_path(&self, path: &str) -> Option<&RegistryHandle> {
        let parts: Vec<&str> = path.split('\\').collect();
        if parts.is_empty() {
            return None;
        }

        // Navigate through the hierarchy
        let mut current = self.next_level_ptr.get(parts[0])?;

        for part in parts.iter().skip(1) {
            current = current.subkeys.get(*part)?;
        }

        // Return default value (empty string) or specific value
        current.registry_handle.get("")
    }

    /// List all subkeys at a given path
    pub fn list_subkeys(&self, path: &str) -> Vec<String> {
        let parts: Vec<&str> = path.split('\\').collect();

        let current = if parts.is_empty() || (parts.len() == 1 && parts[0].is_empty()) {
            // Root level
            return self.next_level_ptr.keys().cloned().collect();
        } else {
            // Navigate to the specified path
            let mut current = match self.next_level_ptr.get(parts[0]) {
                Some(c) => c,
                None => return Vec::new(),
            };

            for part in parts.iter().skip(1) {
                current = match current.subkeys.get(*part) {
                    Some(c) => c,
                    None => return Vec::new(),
                };
            }

            current
        };

        current.subkeys.keys().cloned().collect()
    }

    /// Check if a key exists at the given path
    pub fn key_exists(&self, path: &str) -> bool {
        self.get_key_by_path(path).is_some()
    }

    /// Get a reference to a key by path
    pub fn get_key_by_path(&self, path: &str) -> Option<&RegisterManager> {
        let parts: Vec<&str> = path.split('\\').collect();
        if parts.is_empty() {
            return None;
        }

        let mut current = self.next_level_ptr.get(parts[0])?;

        for part in parts.iter().skip(1) {
            current = current.subkeys.get(*part)?;
        }

        Some(current)
    }
}

// Add helper methods to HiveKey for better integration
impl<'a, R: Read + Seek> HiveKey<'a, R> {
    /// Non-generic version to get RegistryValue directly
    pub fn get_key_value_as_registry(
        &mut self,
        name: &str,
    ) -> Result<Option<RegistryValue>, HiveError> {
        // Use the existing generic method but with RegistryValue type
        self.get_key_value_wrap(name)
    }

    /// Get a subkey by name
    pub fn get_subkey_by_name(&mut self, name: &str) -> Result<Option<HiveKey<'_, R>>, HiveError> {
        let offsets_offset = self.base_offset + self.key_block.subkeys_offset as u64;
        let offsets = crate::emu::object_handle::hive_parser::Offsets::read_from_file(
            self.reader,
            offsets_offset,
        )?;

        for i in 0..self.key_block.subkey_count {
            let offset_entry_offset = offsets_offset + 16 + (i as u64 * 8);
            self.reader
                .seek(std::io::SeekFrom::Start(offset_entry_offset))?;
            let subkey_offset = self.reader.read_i32::<byteorder::LittleEndian>()? as u64;

            if subkey_offset == 0 {
                continue;
            }

            let subkey_abs_offset = self.base_offset + subkey_offset;
            let subkey = KeyBlock::read_from_reader(self.reader, subkey_abs_offset)?;

            let subkey_name = subkey.get_name()?;
            if subkey_name == name {
                return Ok(Some(HiveKey::new(subkey, self.base_offset, self.reader)));
            }
        }

        Ok(None)
    }
}

// Add this method to HiveParser to expose the root key
impl<R: Read + Seek> HiveParser<R> {
    pub fn get_root_key(&mut self) -> Result<HiveKey<'_, R>, HiveError> {
        let main_key_offset = self.base_offset + 0x20;
        let root_key_block = KeyBlock::read_from_reader(&mut self.reader, main_key_offset)?;
        Ok(HiveKey::new(
            root_key_block,
            self.base_offset,
            &mut self.reader,
        ))
    }
}

// Example usage:
/*
fn main() -> Result<(), HiveError> {
    // Load registry hive
    let root_manager = RootRegistryManager::from_hive_file("C:\\Windows\\System32\\config\\SOFTWARE")?;

    // Check if a key exists
    if root_manager.key_exists("Microsoft\\Windows\\CurrentVersion") {
        println!("Key exists!");
    }

    // List subkeys
    let subkeys = root_manager.list_subkeys("Microsoft\\Windows");
    for subkey in subkeys {
        println!("Subkey: {}", subkey);
    }

    // Get a value
    if let Some(handle) = root_manager.get_value_by_path("Microsoft\\Windows\\CurrentVersion\\ProgramFilesDir") {
        match &handle.registry_value {
            RegistryValue::String(s) => println!("Program Files: {}", s),
            _ => println!("Different value type"),
        }
    }

    Ok(())
}
*/
