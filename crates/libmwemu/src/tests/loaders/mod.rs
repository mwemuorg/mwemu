use std::env;
use std::fs;
use std::path::PathBuf;

mod elf;
mod macho;
mod pe;

const HELLO_WORLD_STEPS: usize = 64;

// The three formats share the C source in examples/hello-world/.
fn write_hello_fixture(name: &str, bytes: &[u8]) -> PathBuf {
    let path = env::temp_dir().join(name);
    fs::write(&path, bytes).expect("write hello-world fixture");
    path
}
