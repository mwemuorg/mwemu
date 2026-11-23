use std::error::Error;
use std::path::{Path, PathBuf};
use std::time::Instant;

use libmwemu::emu64;

const INSTRUCTIONS: u64 = 20_000_000;

fn main() -> Result<(), Box<dyn Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let dir = std::env::var_os("MWEMU_TEST_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("test"));
    let image = root.join(dir).join("exe64win_enigma.bin");
    let maps = root.join("maps/windows/x86_64");
    let start = Instant::now();
    let mut emu = emu64();
    emu.cfg.console_enabled = false;
    emu.cfg.winver = None;
    emu.cfg.set_maps_folder(maps.to_string_lossy());
    emu.set_seed(0);
    emu.load_code(image.to_str().ok_or("non-UTF-8 fixture path")?)?;
    emu.run_to(INSTRUCTIONS)?;
    let elapsed = start.elapsed();
    assert!(
        emu.pos >= INSTRUCTIONS,
        "guest stopped at instruction {}",
        emu.pos
    );
    println!("{} guest instructions in {elapsed:.3?}", emu.pos);
    Ok(())
}
