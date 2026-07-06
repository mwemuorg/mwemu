use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::emu::Emu;

/// Candidate names of the 7-Zip CLI, in order of preference. The full `7z`/`7zz`
/// builds understand ISO9660+UDF and WIM/ESD; `7za` is the fallback.
const SEVENZIP_BINS: [&str; 3] = ["7z", "7zz", "7za"];

/// Path inside a Windows install ISO that holds the OS image. Newer media ship
/// an LZMS-compressed `install.esd`; older ones a `install.wim`.
const INSTALL_IMAGES: [&str; 2] = ["sources/install.wim", "sources/install.esd"];

impl Emu {
    /// Build a maps folder out of a real Windows installation ISO and use it for
    /// the emulation. The genuine `System32` DLLs are pulled straight from the
    /// `sources/install.wim` / `install.esd` image inside the ISO (without
    /// mounting it) using the `7z` CLI, and overlaid on top of a copy of the
    /// default maps folder so mwemu's own support files (banzai.csv, loader.exe,
    /// the *.nls tables, …) stay in place.
    ///
    /// The user must download the ISO manually; mwemu never fetches it.
    pub fn set_maps_from_iso(&mut self, iso_path: &str) -> Result<(), Box<dyn Error>> {
        let iso = Path::new(iso_path);
        if !iso.is_file() {
            return Err(format!("ISO not found: {}", iso_path).into());
        }

        let sevenzip = find_sevenzip().ok_or(
            "the `7z` tool is required for --iso but was not found in PATH \
             (install p7zip, e.g. `pacman -S p7zip` or `apt install p7zip-full`)",
        )?;

        // Which architecture's system files do we need, and from which directory
        // of the Windows image? On a 64-bit install the 32-bit DLLs live in
        // SysWOW64, the 64-bit ones in System32.
        let (arch_sub, win_dir) = if self.cfg.arch.is_x64() {
            ("x86_64", "System32")
        } else if self.cfg.arch.is_x86() {
            ("x86", "SysWOW64")
        } else if self.cfg.arch.is_aarch64() {
            ("aarch64", "System32")
        } else {
            return Err("--iso only supports x86, x86_64 and aarch64 Windows".into());
        };

        // Make sure the default maps folder for this arch is present (download it
        // on demand if needed) so we can clone its support files.
        let default_folder = self.ensure_default_maps(arch_sub)?;

        let dest = PathBuf::from(format!("maps/iso/{}/", arch_sub));
        let iso_len = iso.metadata()?.len();

        if iso_cache_is_fresh(&dest, iso_path, iso_len) {
            log::trace!("--iso: reusing cached system32 at {}", dest.display());
            self.set_maps_folder(dest.to_str().unwrap());
            return Ok(());
        }

        log::trace!(
            "--iso: building {} maps from {} (this can take a moment)",
            arch_sub,
            iso_path
        );

        // Start from a clone of the default maps so all of mwemu's non-DLL
        // support files are available, then overlay the real DLLs on top.
        clone_dir(&default_folder, &dest)?;

        // The set of DLLs mwemu actually loads is exactly the *.dll files that
        // ship in the default maps folder — extract only those from the ISO.
        let targets = wanted_dlls(&default_folder)?;
        if targets.is_empty() {
            return Err(format!(
                "no .dll files found in default maps folder {}",
                default_folder.display()
            )
            .into());
        }

        // 1) Pull the install image out of the ISO without mounting it.
        let tmp = PathBuf::from(format!("maps/iso/.tmp_{}/", arch_sub));
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp)?;
        let image = extract_install_image(&sevenzip, iso_path, &tmp)?;

        // 2) Find the OS image index that has the most of the DLLs we want, then
        //    extract just those files from it.
        let chosen = pick_image_files(&sevenzip, &image, win_dir, &targets)?;
        if chosen.is_empty() {
            let _ = fs::remove_dir_all(&tmp);
            return Err(format!(
                "no {}/{} DLLs found inside the install image of {}",
                win_dir, "*.dll", iso_path
            )
            .into());
        }

        let extracted_dir = tmp.join("sys");
        fs::create_dir_all(&extracted_dir)?;
        let copied = extract_and_overlay(&sevenzip, &image, &chosen, &extracted_dir, &dest)?;

        let _ = fs::remove_dir_all(&tmp);
        write_cache_marker(&dest, iso_path, iso_len)?;

        log::trace!(
            "--iso: overlaid {} genuine DLLs from {} into {}",
            copied,
            win_dir,
            dest.display()
        );

        self.set_maps_folder(dest.to_str().unwrap());
        Ok(())
    }

    /// Resolve (and download if missing) the default maps folder for `arch_sub`.
    fn ensure_default_maps(&mut self, arch_sub: &str) -> Result<PathBuf, Box<dyn Error>> {
        // Prefer an already-present folder; otherwise let set_maps_folder fetch it.
        let candidates = [
            format!("maps/windows/{}/", arch_sub),
            format!("../../maps/windows/{}/", arch_sub),
        ];
        for c in &candidates {
            if Path::new(c).join("kernel32.dll").is_file() {
                return Ok(PathBuf::from(c));
            }
        }
        // Not present anywhere: trigger the on-demand download into the repo path.
        let folder = format!("maps/windows/{}/", arch_sub);
        self.set_maps_folder(&folder);
        Ok(PathBuf::from(folder))
    }
}

/// Locate a usable 7-Zip binary in PATH.
fn find_sevenzip() -> Option<String> {
    for bin in SEVENZIP_BINS {
        // `7z` with no args prints its banner and exits; a successful spawn means
        // it exists. We don't care about the exit code.
        if Command::new(bin)
            .arg("i")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .is_ok()
        {
            return Some(bin.to_string());
        }
    }
    None
}

/// True when `dest` already holds a system32 built from this exact ISO.
fn iso_cache_is_fresh(dest: &Path, iso_path: &str, iso_len: u64) -> bool {
    let marker = dest.join(".mwemu-iso");
    let Ok(content) = fs::read_to_string(&marker) else {
        return false;
    };
    content.trim() == cache_marker_value(iso_path, iso_len) && dest.join("kernel32.dll").is_file()
}

fn cache_marker_value(iso_path: &str, iso_len: u64) -> String {
    format!("{}\t{}", iso_path, iso_len)
}

fn write_cache_marker(dest: &Path, iso_path: &str, iso_len: u64) -> Result<(), Box<dyn Error>> {
    fs::write(
        dest.join(".mwemu-iso"),
        cache_marker_value(iso_path, iso_len),
    )?;
    Ok(())
}

/// Copy every regular file from `src` into `dst` (the maps folders are flat).
fn clone_dir(src: &Path, dst: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// Lower-cased set of *.dll filenames present in the default maps folder.
fn wanted_dlls(folder: &Path) -> Result<std::collections::HashSet<String>, Box<dyn Error>> {
    let mut set = std::collections::HashSet::new();
    for entry in fs::read_dir(folder)? {
        let name = entry?.file_name().to_string_lossy().to_ascii_lowercase();
        if name.ends_with(".dll") {
            set.insert(name);
        }
    }
    Ok(set)
}

/// Extract `sources/install.wim` or `install.esd` from the ISO into `tmp`,
/// returning the path of the extracted image file.
fn extract_install_image(
    sevenzip: &str,
    iso_path: &str,
    tmp: &Path,
) -> Result<PathBuf, Box<dyn Error>> {
    for image in INSTALL_IMAGES {
        let out = run_7z(
            sevenzip,
            &[
                "e",
                iso_path,
                &format!("-o{}", tmp.display()),
                image,
                "-aoa",
                "-y",
            ],
        );
        // `7z e` flattens, so the file lands as just its basename.
        let basename = Path::new(image).file_name().unwrap();
        let candidate = tmp.join(basename);
        if candidate.is_file() {
            return Ok(candidate);
        }
        // If the run failed for another reason, surface it on the last attempt.
        let _ = out;
    }
    Err(format!(
        "could not extract {} from {} — is it a Windows installation ISO?",
        INSTALL_IMAGES.join(" or "),
        iso_path
    )
    .into())
}

/// List the image, and among all WIM image indices pick the one containing the
/// most of the wanted DLLs (the full OS edition, not the WinPE setup images).
/// Returns the exact stored paths to extract from that image.
fn pick_image_files(
    sevenzip: &str,
    image: &Path,
    win_dir: &str,
    targets: &std::collections::HashSet<String>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let listing = run_7z(sevenzip, &["l", "-slt", image.to_str().unwrap()])?;

    // 7z on Linux always reports WIM paths with `/` separators, e.g.
    // `2/Windows/System32/kernel32.dll`, and only matches `/`-style patterns on
    // extraction — so we keep the raw path for 7z and normalise only for parsing.
    let needle = format!("/{}/", win_dir).to_ascii_lowercase();
    // image index -> list of raw stored paths (as 7z expects them on extraction)
    let mut by_image: HashMap<String, Vec<String>> = HashMap::new();

    for line in listing.lines() {
        let line = line.trim();
        let Some(path) = line.strip_prefix("Path = ") else {
            continue;
        };
        let lower = path.replace('\\', "/").to_ascii_lowercase();
        if !lower.contains(&needle) {
            continue;
        }
        let Some(file) = lower.rsplit('/').next() else {
            continue;
        };
        if !targets.contains(file) {
            continue;
        }
        // First path component is the WIM image index ("1", "2", …).
        let image_idx = lower.split('/').next().unwrap_or("").to_string();
        by_image
            .entry(image_idx)
            .or_default()
            .push(path.to_string());
    }

    Ok(by_image
        .into_values()
        .max_by_key(|files| files.len())
        .unwrap_or_default())
}

/// Extract the chosen files from the image (flattened into `extracted_dir`) and
/// copy each one, lower-cased, into the destination maps folder. Returns the
/// number of DLLs successfully overlaid.
fn extract_and_overlay(
    sevenzip: &str,
    image: &Path,
    files: &[String],
    extracted_dir: &Path,
    dest: &Path,
) -> Result<usize, Box<dyn Error>> {
    let mut args: Vec<String> = vec![
        "e".into(),
        image.to_string_lossy().into_owned(),
        format!("-o{}", extracted_dir.display()),
        "-aoa".into(),
        "-y".into(),
    ];
    // 7z matches these as path patterns inside the archive.
    args.extend(files.iter().cloned());
    let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    run_7z(sevenzip, &arg_refs)?;

    let mut copied = 0usize;
    for stored in files {
        let basename = stored.rsplit(['/', '\\']).next().unwrap_or(stored);
        let from = extracted_dir.join(basename);
        if from.is_file() {
            let to = dest.join(basename.to_ascii_lowercase());
            if fs::copy(&from, &to).is_ok() {
                copied += 1;
            }
        }
    }
    Ok(copied)
}

/// Run a 7z command, returning stdout on success or an error carrying stderr.
fn run_7z(sevenzip: &str, args: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = Command::new(sevenzip).args(args).output()?;
    if !output.status.success() {
        return Err(format!(
            "`{} {}` failed: {}",
            sevenzip,
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        )
        .into());
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}
