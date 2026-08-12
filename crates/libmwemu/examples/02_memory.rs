//! Map guest memory, write to it, read it back, and search it.
//!
//! This is the API you reach for when you want to seed a buffer before running
//! code, or dig a decrypted string out afterwards.
//!
//! Run with:
//! ```sh
//! cargo run -p libmwemu --features examples --example 02_memory
//! ```

use libmwemu::emu64;
use libmwemu::maps::mem64::Permission;

fn main() {
    let mut emu = emu64();

    // Reserve a region at a fixed address. Maps are named, which is how you
    // find them again (and how they show up in dumps).
    let base = 0x4000_0000;
    emu.maps
        .create_map("scratch", base, 0x1000, Permission::READ_WRITE)
        .expect("cannot create map");

    // Scalar writes. There are matching read_* / write_* helpers for byte,
    // word, dword and qword.
    emu.maps.write_qword(base, 0x1122_3344_5566_7788);
    emu.maps.write_dword(base + 8, 0xdead_beef);

    println!(
        "qword @ 0x{:x} = 0x{:x}",
        base,
        emu.maps.read_qword(base).unwrap()
    );
    println!(
        "dword @ 0x{:x} = 0x{:x}",
        base + 8,
        emu.maps.read_dword(base + 8).unwrap()
    );

    // Strings are written NUL-terminated and read back the same way.
    emu.maps
        .write_string(base + 0x100, "http://example.com/payload");
    let recovered = emu.maps.read_string(base + 0x100);
    println!("string @ 0x{:x} = {:?}", base + 0x100, recovered);

    // Search the whole address space for a substring. `search_string` returns
    // every address where it occurs, so this is how you locate a C2 URL that
    // the sample decrypted at runtime.
    if let Some(hits) = emu.maps.search_string("http://", "scratch") {
        for addr in hits {
            println!("found http:// at 0x{:x}", addr);
        }
    }

    // Byte-pattern search inside a named map. `search_spaced_bytes_in_all`
    // takes a "88 77 66 55" string and sweeps every map instead.
    for addr in emu
        .maps
        .search_bytes(vec![0x88, 0x77, 0x66, 0x55], "scratch")
    {
        println!("found byte pattern at 0x{:x}", addr);
    }

    // Ask what a given address belongs to.
    if let Some(name) = emu.maps.get_addr_name(base + 0x10) {
        println!("0x{:x} lives in map {:?}", base + 0x10, name);
    }

    // Allocate without caring where it lands; `alloc` only picks an address,
    // `map` picks one and creates the region in a single call.
    let heap = emu.maps.map("myheap", 0x2000, Permission::READ_WRITE);
    println!("allocated 0x2000 bytes at 0x{:x}", heap);
}
