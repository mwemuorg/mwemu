//! Linux guest programs built from source with host tools.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::rc::Rc;

use crate::emu64;
use crate::syscall::linux::constants::syscalls::NR64_WRITE;
use crate::tests::helpers;

fn build_go_runtime(dir: &Path) -> PathBuf {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/tests/fixtures/linux_go_runtime.go");
    let bin = dir.join("sample");
    let output = Command::new("go")
        .current_dir(dir)
        .args(["build", "-ldflags=-s -w", "-o"])
        .arg(&bin)
        .arg(&src)
        .env("CGO_ENABLED", "0")
        .env("GOOS", "linux")
        .env("GOARCH", "amd64")
        .env("GOAMD64", "v1")
        .env("GOTOOLCHAIN", "local")
        .env("GOWORK", "off")
        .env("GOPROXY", "off")
        .output()
        .expect("Go compiler required in PATH");
    assert!(
        output.status.success(),
        "Go build failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    bin
}

#[test]
fn go_goroutines_timers_and_exec() {
    helpers::setup();
    let dir = tempfile::tempdir().expect("scratch dir");
    let bin = build_go_runtime(dir.path());

    let mut emu = emu64();
    emu.cfg.console_enabled = false;
    emu.max_pos = Some(50_000_000);
    emu.load_code(bin.to_str().expect("utf-8 path"))
        .expect("load Go ELF");
    assert!(emu.os.is_linux());

    let output = Rc::new(RefCell::new([Vec::new(), Vec::new()]));
    let sink = Rc::clone(&output);
    emu.hooks.on_syscall(move |emu, nr| {
        if nr == NR64_WRITE && matches!(emu.regs().rdi, 1 | 2) {
            let (buf, len) = (emu.regs().rsi, emu.regs().rdx);
            let mut output = sink.borrow_mut();
            let stream = &mut output[emu.regs().rdi as usize - 1];
            for i in 0..len {
                stream.push(emu.maps.read_byte(buf + i).expect("read Go output"));
            }
        }
        true
    });

    let result = emu.run(None);
    let output = output.borrow();
    let stdout = String::from_utf8_lossy(&output[0]);
    let stderr = String::from_utf8_lossy(&output[1]);
    let diagnostic = format!(
        "result={result:?}, pos={}, pc=0x{:x}, stderr={stderr:?}",
        emu.pos,
        emu.pc()
    );
    assert!(result.is_ok(), "{diagnostic}");
    assert!(
        emu.max_pos.is_none_or(|cap| emu.pos < cap),
        "the Go program hit the instruction cap: {diagnostic}"
    );
    assert_eq!(
        stdout, "squares: [0 1 4 9 16 25 36 49]\ndone\n",
        "{diagnostic}"
    );
    // exit_group status lands in rdi.
    assert_eq!(emu.regs().rdi, 7, "exit status");
    assert!(
        emu.threads.len() > 1,
        "the runtime should have cloned M threads"
    );
}
