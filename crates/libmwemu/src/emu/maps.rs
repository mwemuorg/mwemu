use std::path::Path;

use crate::emu::Emu;
use crate::maps::mem64::Permission;

impl Emu {
    /// For simulating a windows process space, select the folder with maps (e.g. maps/windows/x86_64/) depending upon the arch, do this before loading the binary.
    pub fn set_maps_folder(&mut self, folder: &str) {
        self.cfg.maps_folder = folder.to_string();

        if self.maps_folder_is_valid(folder) {
            return;
        }

        // Create the directory and let on-demand fetching from the symbol
        // server fill it (x64 DLLs). For 32-bit maps the user must supply
        // them manually with --maps or --iso.
        log::trace!(
            "Maps folder '{}' not found or incomplete; creating it",
            folder
        );
        let _ = std::fs::create_dir_all(folder);
    }

    /// Check if maps folder exists and contains essential files
    fn maps_folder_is_valid(&self, folder: &str) -> bool {
        let folder_path = Path::new(folder);
        if !folder_path.exists() {
            return false;
        }

        let essential_files = if !self.cfg.arch.is_64bits() {
            if self.cfg.skip_unimplemented {
                vec!["ntdll.dll", "kernel32.dll", "banzai.csv"]
            } else {
                vec!["ntdll.dll", "kernel32.dll"]
            }
        } else {
            vec!["ntdll.dll", "kernel32.dll"]
        };

        for file in essential_files {
            let file_path = folder_path.join(file);
            if !file_path.exists() {
                log::trace!(
                    "Essential file '{}' missing from maps folder",
                    file_path.display()
                );
                return false;
            }
        }

        true
    }

    /// Get the base address of the code, if code map doesn't exist yet will return None.
    pub fn get_base_addr(&self) -> Option<u64> {
        //TODO: fix this, now there is no code map.
        let map = self.maps.get_map_by_name("code")?;

        Some(map.get_base())
    }

    /// From a file-path this returns the filename with no path and no extension.
    /// Handles both `/` and `\` path separators so Windows CI paths work.
    pub fn filename_to_mapname(&self, filename: &str) -> String {
        std::path::Path::new(filename)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(filename)
            .to_string()
    }

    /// Remove from the memory the map name provided.
    pub fn free(&mut self, name: &str) {
        self.maps.free(name);
    }

    /// This find an empty space on the memory of selected size
    /// and also creates a map there.
    pub fn alloc(&mut self, name: &str, size: u64, permission: Permission) -> u64 {
        let addr = match self.maps.alloc(size) {
            Some(a) => a,
            None => {
                log::trace!("low memory");
                return 0;
            }
        };
        if let Err(e) = self.maps.create_map(name, addr, size, permission) {
            log::error!("cannot create map '{}' from alloc api: {}", name, e);
            return 0;
        }
        addr
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn maps_folder_requires_banzai_csv_only_when_skip_unimplemented() {
        let dir = tempfile::tempdir().expect("temp maps dir");
        fs::write(dir.path().join("ntdll.dll"), []).expect("ntdll fixture");
        fs::write(dir.path().join("kernel32.dll"), []).expect("kernel32 fixture");
        let folder = dir.path().to_str().expect("utf8 temp path");

        let mut emu = crate::emu32();
        emu.cfg.skip_unimplemented = false;
        assert!(emu.maps_folder_is_valid(folder));

        emu.cfg.skip_unimplemented = true;
        assert!(!emu.maps_folder_is_valid(folder));

        fs::write(dir.path().join("banzai.csv"), []).expect("banzai fixture");
        assert!(emu.maps_folder_is_valid(folder));
    }
}
