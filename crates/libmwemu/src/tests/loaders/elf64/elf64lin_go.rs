//! A static Go binary end to end: the runtime's heap reservations (large
//! PROT_NONE mmaps, partial munmaps), its M threads (`clone`, `futex`,
//! `exit`), the netpoller it drives timers through (`epoll_*`, `eventfd`),
//! `clock_gettime` for `time.Sleep`, and `os/exec`'s vfork + execve.
//!
//! The sample is built here with the host's `go` toolchain (static, no cgo),
//! so the test needs no bundle and skips itself where `go` is absent.

const SOURCE: &str = r#"
package main

import (
	"fmt"
	"os"
	"os/exec"
	"sort"
	"sync"
	"time"
)

func worker(id int, jobs <-chan int, results chan<- int, wg *sync.WaitGroup) {
	defer wg.Done()
	for j := range jobs {
		time.Sleep(time.Millisecond)
		results <- j * j
	}
}

func main() {
	jobs := make(chan int, 16)
	results := make(chan int, 16)
	var wg sync.WaitGroup
	for w := 0; w < 4; w++ {
		wg.Add(1)
		go worker(w, jobs, results, &wg)
	}
	for i := 0; i < 8; i++ {
		jobs <- i
	}
	close(jobs)
	wg.Wait()
	close(results)
	var squares []int
	for r := range results {
		squares = append(squares, r)
	}
	sort.Ints(squares)
	fmt.Println("squares:", squares)
	cmd := exec.Command("/bin/echo", "child")
	cmd.Stdin = os.Stdin
	if _, err := cmd.Output(); err != nil {
		fmt.Println("exec error:", err)
	}
	fmt.Println("done")
	os.Exit(7)
}
"#;

fn build_sample(dir: &std::path::Path) -> Option<std::path::PathBuf> {
    use std::process::Command;

    let src = dir.join("main.go");
    let bin = dir.join("sample");
    std::fs::write(&src, SOURCE).expect("write Go source");
    let output = Command::new("go")
        .args(["build", "-ldflags=-s -w", "-o"])
        .arg(&bin)
        .arg(&src)
        .env("CGO_ENABLED", "0")
        .env("GOOS", "linux")
        .env("GOARCH", "amd64")
        .env("GOAMD64", "v1")
        .env("GOTOOLCHAIN", "local")
        .output();
    let output = match output {
        Ok(output) => output,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return None,
        Err(error) => panic!("start Go compiler: {error}"),
    };
    assert!(
        output.status.success(),
        "Go build failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    Some(bin)
}

#[test]
fn elf64lin_go_goroutines_timers_exec() {
    use crate::syscall::linux::constants::syscalls::NR64_WRITE;
    use crate::tests::helpers;
    use crate::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    helpers::setup();
    let dir = tempfile::tempdir().expect("scratch dir");
    let Some(bin) = build_sample(dir.path()) else {
        eprintln!("[skip] elf64lin_go: no `go` toolchain in PATH");
        return;
    };

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
