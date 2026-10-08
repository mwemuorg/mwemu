//! A static Go binary end to end: the runtime's heap reservations (large
//! PROT_NONE mmaps, partial munmaps), its M threads (`clone`, `futex`,
//! `exit`), the netpoller it drives timers through (`epoll_*`, `eventfd`),
//! `clock_gettime` for `time.Sleep`, and `os/exec`'s vfork + execve.
//!
//! The sample is built here with the host's `go` toolchain (static, no cgo),
//! so the test needs no bundle and skips itself where `go` is absent.

#[cfg(not(target_os = "windows"))]
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
	if _, err := exec.Command("/bin/echo", "child").Output(); err != nil {
		fmt.Println("exec error:", err)
	}
	fmt.Println("done")
	os.Exit(7)
}
"#;

#[cfg(not(target_os = "windows"))]
fn build_sample(dir: &std::path::Path) -> Option<std::path::PathBuf> {
    use std::process::Command;

    let src = dir.join("main.go");
    let bin = dir.join("sample");
    std::fs::write(&src, SOURCE).ok()?;
    let status = Command::new("go")
        .args(["build", "-ldflags=-s -w", "-o"])
        .arg(&bin)
        .arg(&src)
        .env("CGO_ENABLED", "0")
        .env("GOOS", "linux")
        .env("GOARCH", "amd64")
        .status()
        .ok()?;
    status.success().then_some(bin)
}

#[test]
#[cfg(not(target_os = "windows"))]
fn elf64lin_go_goroutines_timers_exec() {
    use crate::tests::helpers;
    use crate::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    helpers::setup();
    let dir = tempfile::tempdir().expect("scratch dir");
    let Some(bin) = build_sample(dir.path()) else {
        eprintln!("[skip] elf64lin_go: no working `go` toolchain in PATH");
        return;
    };

    let mut emu = emu64();
    emu.max_pos = Some(50_000_000);
    emu.load_code(bin.to_str().expect("utf-8 path"));
    assert!(emu.os.is_linux());

    // Collect what the program writes to stdout (write(1, buf, len)).
    let stdout = Rc::new(RefCell::new(Vec::new()));
    let sink = Rc::clone(&stdout);
    emu.hooks.on_syscall(move |emu, nr| {
        if nr == 1 && emu.regs().rdi == 1 {
            let (buf, len) = (emu.regs().rsi, emu.regs().rdx);
            let mut out = sink.borrow_mut();
            for i in 0..len {
                out.push(emu.maps.read_byte(buf + i).unwrap_or(0));
            }
        }
        true
    });

    let _ = emu.run(None);

    assert!(
        emu.max_pos.map(|cap| emu.pos < cap).unwrap_or(true),
        "the Go program did not terminate: hit the instruction cap at pos {}",
        emu.pos
    );
    let out = String::from_utf8_lossy(&stdout.borrow()).to_string();
    assert_eq!(out, "squares: [0 1 4 9 16 25 36 49]\ndone\n");
    // exit_group status lands in rdi.
    assert_eq!(emu.regs().rdi, 7, "exit status");
    assert!(
        emu.threads.len() > 1,
        "the runtime should have cloned M threads"
    );
}
