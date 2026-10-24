#![no_main]
use libfuzzer_sys::fuzz_target;
use libmwemu::emu::Emu;
use std::panic;
use std::sync::Once;

static INIT: Once = Once::new();

fuzz_target!(|data: &[u8]| {
    INIT.call_once(|| {
        panic::set_hook(Box::new(|_| {}));
    });

    if data.len() < 2 {
        return;
    }
    let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        let mut emu = Emu::new(libmwemu::arch::Arch::X86);
        let _ = emu.load_code_bytes(data);
    }));
});
