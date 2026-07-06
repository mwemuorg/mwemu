//! End-to-end loader test: bind a real PE's imports through a mock `PeLoader`
//! (an in-memory backend), proving the generic loader works without an
//! emulator and that `import_addr_to_name` is reversible from `iat_names`.

use std::collections::HashMap;

use rs_header::pe::pe64::PE64;
use rs_header::pe::PeLoader;

/// Minimal in-memory PeLoader: records writes and hands out fake, stable
/// addresses for resolved imports.
struct Mock {
    mem: HashMap<u64, u8>,
    next: u64,
    resolved: HashMap<String, u64>,
}

impl Mock {
    fn new() -> Self {
        Mock {
            mem: HashMap::new(),
            next: 0x7000_0000,
            resolved: HashMap::new(),
        }
    }
}

impl PeLoader for Mock {
    fn is_mapped(&self, _addr: u64) -> bool {
        true
    }
    fn write_bytes(&mut self, addr: u64, data: &[u8]) -> bool {
        for (i, b) in data.iter().enumerate() {
            self.mem.insert(addr + i as u64, *b);
        }
        true
    }
    fn write_dword(&mut self, addr: u64, val: u32) -> bool {
        self.write_bytes(addr, &val.to_le_bytes())
    }
    fn write_qword(&mut self, addr: u64, val: u64) -> bool {
        self.write_bytes(addr, &val.to_le_bytes())
    }
    fn load_library(&mut self, _libname: &str) -> u64 {
        0x1000 // non-zero = "loaded"
    }
    fn resolve_api_name(&mut self, name: &str) -> u64 {
        self.resolve_api_name_in_module("?", name)
    }
    fn resolve_api_name_in_module(&mut self, _module: &str, name: &str) -> u64 {
        if let Some(a) = self.resolved.get(name) {
            return *a;
        }
        let a = self.next;
        self.next += 0x10;
        self.resolved.insert(name.to_string(), a);
        a
    }
    fn search_api_name(&mut self, name: &str) -> (u64, String, String) {
        (self.resolve_api_name(name), "mock".to_string(), name.to_string())
    }
}

static LOADER64: &[u8] = include_bytes!("fixtures/loader64.exe");

#[test]
fn iat_binding_generic_roundtrip() {
    let mut pe = PE64::parse("loader64.exe", LOADER64);
    let mut mock = Mock::new();
    let base = 0x4000_0000u64;

    // The generic loader runs against the mock backend without any emulator.
    pe.iat_binding(LOADER64, &mut mock, base);
    pe.delay_load_binding(LOADER64, &mut mock, base);
    pe.apply_relocations(LOADER64, &mut mock, base); // must not panic

    // Every recorded import is reversible by address — no file bytes needed.
    for (&addr, full) in pe.iat_names.iter() {
        let name = full.split_once('!').expect("dll!name").1;
        assert_eq!(pe.import_addr_to_name(addr), name);
        assert_eq!(pe.import_addr_to_dll_and_name(addr), *full);
    }

    // Resolving a missing address yields empty (not a panic).
    assert_eq!(pe.import_addr_to_name(0xdead_beef), "");
}
