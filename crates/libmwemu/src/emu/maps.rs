use std::{fs, io, path::Path};

use crate::emu::Emu;
use crate::maps::mem64::Permission;

impl Emu {
    /// For simulating a windows process space, select the folder with maps (e.g. maps/windows/x86_64/) depending upon the arch, do this before loading the binary.
    pub fn set_maps_folder(&mut self, folder: &str) {
        //let mut f = folder.to_string();
        //f.push('/');
        self.cfg.maps_folder = folder.to_string();

        if self.maps_folder_is_valid(folder) {
            return;
        }

        // 32-bit maps still need the legacy release zip (no symbol-server support
        // for them yet). Everything else (x64 Windows) is fine empty: the loader
        // auto-fetches each missing DLL from the symbol server on demand
        // (`ensure_maps_dll`), so just make sure the directory exists.
        let is_32bit = folder.contains("windows/x86") && !folder.contains("x86_64");
        if is_32bit {
            log::trace!(
                "Maps folder '{}' incomplete, attempting legacy download...",
                folder
            );
            if let Err(e) = self.download_and_extract_maps(folder) {
                log::error!("Failed to download 32-bit maps '{}': {}", folder, e);
                panic!("Cannot obtain 32-bit Windows maps. Supply your own with `--maps <dir>`.");
            }
            return;
        }

        // x64 (or custom) folder: create it and let on-demand fetching fill it.
        log::trace!(
            "Maps folder '{}' empty; DLLs will be auto-fetched from the symbol server on demand",
            folder
        );
        let _ = std::fs::create_dir_all(folder);
    }

    /// Check if maps folder exists and contains essential files
    fn maps_folder_is_valid(&self, folder: &str) -> bool {
        let folder_path = Path::new(folder);
        if !folder_path.exists() {
            println!("folder doesnt exist");
            return false;
        }

        // Check for essential files based on architecture
        let essential_files = if folder.contains("32") {
            println!("self.cfg.emulate_winapi: {}", self.cfg.emulate_winapi);
            if self.cfg.emulate_winapi {
                vec!["ntdll.dll", "kernel32.dll"]
            } else {
                vec!["ntdll.dll", "kernel32.dll", "banzai.csv"]
            }
        } else {
            vec!["ntdll.dll", "kernel32.dll"]
        };

        for file in essential_files {
            let file_path = folder_path.join(file);
            if !file_path.exists() {
                println!("essential file missing: {}", file_path.display());
                log::trace!(
                    "Essential file '{}' missing from maps folder",
                    file_path.display()
                );
                return false;
            }
        }

        true
    }

    /// Download and extract maps folder from specific URL
    fn download_and_extract_maps(&self, folder: &str) -> Result<(), Box<dyn std::error::Error>> {
        // x86_64 maps are fetched from the symbol server in set_maps_folder
        // (--winver) and never reach here. 32-bit maps don't have symbol-server
        // support yet, so they still use the legacy release zip.
        let url = match folder {
            f if f.contains("windows/x86") && !f.contains("x86_64") => {
                "https://github.com/sha0coder/mwemu/releases/download/maps/maps32.zip"
            }
            _ => return Err(format!("Unknown maps folder: {}", folder).into()),
        };

        log::trace!(
            "Downloading {} from GitHub releases... (this may take a moment)",
            folder
        );

        // Download the file using ureq
        // Note: To reduce TLS verbosity, set RUST_LOG=info instead of debug
        let response = ureq::get(url)
            .timeout(std::time::Duration::from_secs(30))
            .call()?;

        if response.status() != 200 {
            return Err(format!("Failed to download: HTTP {}", response.status()).into());
        }

        let mut bytes = Vec::new();
        response.into_reader().read_to_end(&mut bytes)?;
        log::trace!("Downloaded {} MB", bytes.len() / 1024 / 1024);

        // Extract the zip file
        let cursor = std::io::Cursor::new(bytes);
        let mut archive = zip::ZipArchive::new(cursor)?;

        log::trace!("Extracting {} files...", archive.len());

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = Path::new(file.name());

            if file.name().ends_with('/') {
                // Create directory
                fs::create_dir_all(outpath)?;
            } else {
                // Create parent directories if they don't exist
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p)?;
                    }
                }

                // Extract file
                let mut outfile = fs::File::create(outpath)?;
                io::copy(&mut file, &mut outfile)?;
            }
        }

        log::trace!("Successfully extracted maps folder: {}", folder);
        Ok(())
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
        self.maps
            .create_map(name, addr, size, permission)
            .expect("cannot create map from alloc api");
        addr
    }
}
