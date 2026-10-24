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

    if data.len() < 4 || data.len() > 4096 {
        return;
    }
    let mut emu = Emu::new(libmwemu::arch::Arch::X86);
    if emu.load_code_bytes(data).is_ok() {
        emu.cfg.verbose = 0;
        emu.cfg.max_instructions = Some(5000);
        let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            let _ = emu.run(None);
        }));
    }
});
