//! Bulk parser corpus test for `rs-header`.
//!
//! Recursively scans one or more corpus directories for PE and ELF files,
//! parses each detected candidate, and validates basic invariants. Parser
//! panics and failed invariants fail the test by default; an opt-in
//! `RS_HEADER_ALLOW_MALFORMED=1` reports malformed/invariant failures but keeps
//! the run green.
//!
//! Run explicitly:
//!
//! ```bash
//! RS_HEADER_CORPUS_DIR=path/to/binaries \
//!   cargo test -p rs-header --test bulk_parse -- --ignored --nocapture
//! ```
//!
//! See `.kilo/plans/1783745443585-bulk-rs-header-corpus-parser-plan.md`.

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};

use rs_header::elf::elf32::Elf32;
use rs_header::elf::elf64::Elf64;
use rs_header::pe::pe32::PE32;
use rs_header::pe::pe64::PE64;

const DETECT_PREFIX_LEN: usize = 512;
const DEFAULT_MAX_FILE_MB: u64 = 256;
const DETAILED_FAILURES_LIMIT: usize = 100;

#[derive(Debug)]
#[allow(dead_code)]
enum FailureKind {
    Panic,
    ParseError,
    Invariant,
    Io,
    Unsupported,
    TooLarge,
}

impl FailureKind {
    fn label(&self) -> &'static str {
        match self {
            FailureKind::Panic => "panic",
            FailureKind::ParseError => "parse_error",
            FailureKind::Invariant => "invariant",
            FailureKind::Io => "io",
            FailureKind::Unsupported => "unsupported",
            FailureKind::TooLarge => "too_large",
        }
    }
}

#[derive(Debug)]
struct Failure {
    path: PathBuf,
    format: &'static str,
    kind: FailureKind,
    message: String,
}

#[derive(Default)]
struct Counts {
    dirs_scanned: usize,
    files_scanned: u64,
    candidate_pe: usize,
    candidate_elf: usize,
    parsed_pe32: usize,
    parsed_pe64: usize,
    parsed_elf32: usize,
    parsed_elf64: usize,
    skipped_too_large: usize,
    skipped_unreadable: usize,
    unknown_class: usize,
    unknown_machine: usize,
    malformed: usize,
    panics: usize,
}

struct Config {
    corpus_dirs: Vec<PathBuf>,
    allow_malformed: bool,
    max_file_bytes: u64,
    limit: Option<usize>,
}

impl Config {
    fn from_env() -> Result<Config, String> {
        let raw = std::env::var("RS_HEADER_CORPUS_DIR")
            .map_err(|_| "RS_HEADER_CORPUS_DIR is not set".to_string())?;
        if raw.trim().is_empty() {
            return Err("RS_HEADER_CORPUS_DIR is empty".to_string());
        }
        let corpus_dirs: Vec<PathBuf> = std::env::split_paths(&raw)
            .filter(|p| !p.as_os_str().is_empty())
            .collect();
        if corpus_dirs.is_empty() {
            return Err("RS_HEADER_CORPUS_DIR produced no usable paths".to_string());
        }

        let allow_malformed = match std::env::var("RS_HEADER_ALLOW_MALFORMED") {
            Ok(v) => v == "1",
            Err(_) => false,
        };

        let max_file_mb: u64 = match std::env::var("RS_HEADER_MAX_FILE_MB") {
            Ok(v) => v.parse().unwrap_or(DEFAULT_MAX_FILE_MB),
            Err(_) => DEFAULT_MAX_FILE_MB,
        };
        let max_file_bytes = max_file_mb.saturating_mul(1024 * 1024);

        let limit = match std::env::var("RS_HEADER_LIMIT") {
            Ok(v) => v.parse().ok(),
            Err(_) => None,
        };

        Ok(Config {
            corpus_dirs,
            allow_malformed,
            max_file_bytes,
            limit,
        })
    }
}

enum Format {
    Pe,
    Elf { class: u8 },
}

/// Inspect the leading bytes to decide whether this file is a PE or ELF
/// candidate, and which sub-format. Returns `None` for files we should skip
/// without further work.
fn detect_format(prefix: &[u8]) -> Option<Format> {
    if prefix.len() < 4 {
        return None;
    }
    if prefix[0] == b'M' && prefix[1] == b'Z' {
        return Some(Format::Pe);
    }
    if prefix[0] == 0x7f && prefix[1] == b'E' && prefix[2] == b'L' && prefix[3] == b'F' {
        let class = *prefix.get(4)?;
        return Some(Format::Elf { class });
    }
    None
}

fn walk(
    root: &Path,
    dirs_scanned: &mut usize,
    files_scanned: &mut u64,
    visited: &mut Vec<PathBuf>,
    out: &mut Vec<PathBuf>,
    max_files: Option<u64>,
) {
    let stack = std::fs::read_dir(root);
    let entries = match stack {
        Ok(it) => it,
        Err(_) => {
            eprintln!("warn: cannot read directory {}", root.display());
            return;
        }
    };
    *dirs_scanned += 1;
    let mut bucket: Vec<PathBuf> = Vec::new();
    for entry in entries.flatten() {
        let p = entry.path();
        let ft = match entry.file_type() {
            Ok(ft) => ft,
            Err(_) => continue,
        };
        if ft.is_dir() {
            bucket.push(p);
        } else if ft.is_file() {
            *files_scanned += 1;
            out.push(p);
            if let Some(limit) = max_files {
                if *files_scanned >= limit {
                    return;
                }
            }
        }
    }
    for d in bucket {
        if !visited.iter().any(|v| v == &d) {
            visited.push(d.clone());
            walk(&d, dirs_scanned, files_scanned, visited, out, max_files);
            if let Some(limit) = max_files {
                if *files_scanned >= limit {
                    return;
                }
            }
        }
    }
}

fn collect_files(cfg: &Config) -> (Vec<PathBuf>, Counts) {
    let mut counts = Counts::default();
    let mut all: Vec<PathBuf> = Vec::new();
    let mut visited: Vec<PathBuf> = Vec::new();
    for root in &cfg.corpus_dirs {
        if !root.exists() {
            eprintln!("warn: corpus dir does not exist: {}", root.display());
            continue;
        }
        let max = cfg.limit.map(|n| n as u64).unwrap_or(u64::MAX);
        walk(
            root,
            &mut counts.dirs_scanned,
            &mut counts.files_scanned,
            &mut visited,
            &mut all,
            Some(max),
        );
    }
    if let Some(limit) = cfg.limit {
        all.truncate(limit);
    }
    (all, counts)
}

fn read_prefix(path: &Path, len: usize) -> Option<Vec<u8>> {
    use std::io::Read;
    let mut f = std::fs::File::open(path).ok()?;
    let mut buf = vec![0u8; len];
    let mut total = 0usize;
    while total < len {
        let n = match f.read(&mut buf[total..]) {
            Ok(n) => n,
            Err(_) => break,
        };
        if n == 0 {
            break;
        }
        total += n;
    }
    buf.truncate(total);
    Some(buf)
}

fn handle_pe(path: &Path, raw: &[u8], failures: &mut Vec<Failure>, counts: &mut Counts) {
    counts.candidate_pe += 1;
    let path_display = path.display().to_string();

    let pe32_ok = PE32::is_pe32(raw);
    let pe64_ok = PE64::is_pe64(raw);

    if pe32_ok && pe64_ok {
        // Both detectors shouldn't agree; skip as unsupported.
        counts.unknown_class += 1;
        failures.push(Failure {
            path: path.to_path_buf(),
            format: "pe",
            kind: FailureKind::Unsupported,
            message: "is_pe32 and is_pe64 both true".into(),
        });
        return;
    }
    if !pe32_ok && !pe64_ok {
        counts.unknown_machine += 1;
        failures.push(Failure {
            path: path.to_path_buf(),
            format: "pe",
            kind: FailureKind::Unsupported,
            message: "unsupported PE machine".into(),
        });
        return;
    }

    if pe64_ok {
        handle_pe64(path, raw, failures, counts);
    } else {
        handle_pe32(path, raw, failures, counts);
    }
    let _ = path_display; // kept for debugging in future extensions
}

fn handle_pe32(path: &Path, raw: &[u8], failures: &mut Vec<Failure>, counts: &mut Counts) {
    let path_display = path.display().to_string();
    let parse_result = catch_unwind(AssertUnwindSafe(|| PE32::parse(&path_display, raw)));
    let pe = match parse_result {
        Ok(pe) => pe,
        Err(_) => {
            counts.panics += 1;
            failures.push(Failure {
                path: path.to_path_buf(),
                format: "pe32",
                kind: FailureKind::Panic,
                message: "PE32::parse panicked".into(),
            });
            return;
        }
    };

    let mut err: Option<String> = None;
    if pe.dos.e_magic != 0x5a4d {
        err = Some(format!("dos.e_magic=0x{:x}", pe.dos.e_magic));
    } else if pe.opt.magic != 0x10b {
        err = Some(format!("opt.magic=0x{:x}", pe.opt.magic));
    }

    if err.is_none() {
        let ns = pe.num_of_sections();
        if ns != pe.fh.number_of_sections as usize {
            err = Some(format!(
                "num_of_sections mismatch: parsed={} fh={}",
                ns, pe.fh.number_of_sections
            ));
        }
    }

    if err.is_none() {
        if (pe.opt.size_of_headers as usize) > raw.len() {
            err = Some(format!(
                "size_of_headers {} > raw.len() {}",
                pe.opt.size_of_headers,
                raw.len()
            ));
        }
    }

    if err.is_none() {
        for i in 0..pe.num_of_sections() {
            let sect = pe.get_section(i);
            let off = sect.pointer_to_raw_data as usize;
            let sz = sect.virtual_size as usize;
            if off > raw.len() {
                err = Some(format!("section {i} raw off {off} > raw.len() {}", raw.len()));
                break;
            }
            if sz > raw.len() {
                err = Some(format!("section {i} virtual_size {sz} > raw.len() {}", raw.len()));
                break;
            }
            if off.checked_add(sz).map_or(true, |sum| sum > raw.len()) {
                err = Some(format!("section {i} off+sz > raw.len() {}", raw.len()));
                break;
            }
            let _ = pe.get_section_ptr_by_name(raw, sect.get_name().as_str())
                .unwrap_or(&[]);
        }
    }

    if let Some(msg) = err {
        counts.malformed += 1;
        failures.push(Failure {
            path: path.to_path_buf(),
            format: "pe32",
            kind: FailureKind::Invariant,
            message: msg,
        });
        return;
    }

    if pe.opt.size_of_headers as usize <= raw.len() {
        let _ = pe.headers(raw);
    }
    counts.parsed_pe32 += 1;
}

fn handle_pe64(path: &Path, raw: &[u8], failures: &mut Vec<Failure>, counts: &mut Counts) {
    let path_display = path.display().to_string();
    let parse_result = catch_unwind(AssertUnwindSafe(|| PE64::parse(&path_display, raw)));
    let pe = match parse_result {
        Ok(pe) => pe,
        Err(_) => {
            counts.panics += 1;
            failures.push(Failure {
                path: path.to_path_buf(),
                format: "pe64",
                kind: FailureKind::Panic,
                message: "PE64::parse panicked".into(),
            });
            return;
        }
    };

    let mut err: Option<String> = None;
    if pe.dos.e_magic != 0x5a4d {
        err = Some(format!("dos.e_magic=0x{:x}", pe.dos.e_magic));
    } else if pe.opt.magic != 0x20b {
        err = Some(format!("opt.magic=0x{:x}", pe.opt.magic));
    }

    if err.is_none() {
        let ns = pe.num_of_sections();
        if ns != pe.fh.number_of_sections as usize {
            err = Some(format!(
                "num_of_sections mismatch: parsed={} fh={}",
                ns, pe.fh.number_of_sections
            ));
        }
    }

    if err.is_none() {
        if (pe.opt.size_of_headers as usize) > raw.len() {
            err = Some(format!(
                "size_of_headers {} > raw.len() {}",
                pe.opt.size_of_headers,
                raw.len()
            ));
        }
    }

    if err.is_none() {
        for i in 0..pe.num_of_sections() {
            let sect = pe.get_section(i);
            let off = sect.pointer_to_raw_data as usize;
            let sz = sect.virtual_size as usize;
            if off > raw.len() {
                err = Some(format!("section {i} raw off {off} > raw.len() {}", raw.len()));
                break;
            }
            if sz > raw.len() {
                err = Some(format!("section {i} virtual_size {sz} > raw.len() {}", raw.len()));
                break;
            }
            if off.checked_add(sz).map_or(true, |sum| sum > raw.len()) {
                err = Some(format!("section {i} off+sz > raw.len() {}", raw.len()));
                break;
            }
            let _ = pe.get_section_ptr_by_name(raw, sect.get_name().as_str())
                .unwrap_or(&[]);
        }
    }

    if let Some(msg) = err {
        counts.malformed += 1;
        failures.push(Failure {
            path: path.to_path_buf(),
            format: "pe64",
            kind: FailureKind::Invariant,
            message: msg,
        });
        return;
    }

    if pe.opt.size_of_headers as usize <= raw.len() {
        let _ = pe.headers(raw);
    }
    counts.parsed_pe64 += 1;
}

fn handle_elf32(path: &Path, raw: &[u8], failures: &mut Vec<Failure>, counts: &mut Counts) {
    let parse_result = catch_unwind(AssertUnwindSafe(|| Elf32::parse(raw)));
    let elf = match parse_result {
        Ok(Ok(elf)) => elf,
        Ok(Err(e)) => {
            counts.malformed += 1;
            failures.push(Failure {
                path: path.to_path_buf(),
                format: "elf32",
                kind: FailureKind::ParseError,
                message: format!("Elf32::parse returned Err: {e}"),
            });
            return;
        }
        Err(_) => {
            counts.panics += 1;
            failures.push(Failure {
                path: path.to_path_buf(),
                format: "elf32",
                kind: FailureKind::Panic,
                message: "Elf32::parse panicked".into(),
            });
            return;
        }
    };

    if elf.elf_hdr.e_ident[..4] != [0x7f, b'E', b'L', b'F'] {
        counts.malformed += 1;
        failures.push(Failure {
            path: path.to_path_buf(),
            format: "elf32",
            kind: FailureKind::Invariant,
            message: "e_ident magic mismatch".into(),
        });
        return;
    }
    if elf.elf_hdr.e_ident[4] != 1 {
        counts.malformed += 1;
        failures.push(Failure {
            path: path.to_path_buf(),
            format: "elf32",
            kind: FailureKind::Invariant,
            message: format!("e_ident class={} != 1", elf.elf_hdr.e_ident[4]),
        });
        return;
    }
    counts.parsed_elf32 += 1;
}

fn handle_elf64(path: &Path, raw: &[u8], failures: &mut Vec<Failure>, counts: &mut Counts) {
    if raw.len() < 5 || raw[4] != 2 {
        counts.unknown_class += 1;
        failures.push(Failure {
            path: path.to_path_buf(),
            format: "elf64",
            kind: FailureKind::Unsupported,
            message: format!("elf class {} not 2", raw.get(4).copied().unwrap_or(0)),
        });
        return;
    }

    let parse_result = catch_unwind(AssertUnwindSafe(|| Elf64::parse(raw)));
    let elf = match parse_result {
        Ok(Ok(elf)) => elf,
        Ok(Err(e)) => {
            counts.malformed += 1;
            failures.push(Failure {
                path: path.to_path_buf(),
                format: "elf64",
                kind: FailureKind::ParseError,
                message: format!("Elf64::parse returned Err: {e}"),
            });
            return;
        }
        Err(_) => {
            counts.panics += 1;
            failures.push(Failure {
                path: path.to_path_buf(),
                format: "elf64",
                kind: FailureKind::Panic,
                message: "Elf64::parse panicked".into(),
            });
            return;
        }
    };

    if elf.elf_hdr.e_ident[..4] != [0x7f, b'E', b'L', b'F'] {
        counts.malformed += 1;
        failures.push(Failure {
            path: path.to_path_buf(),
            format: "elf64",
            kind: FailureKind::Invariant,
            message: "e_ident magic mismatch".into(),
        });
        return;
    }
    if elf.elf_hdr.e_ident[4] != 2 {
        counts.malformed += 1;
        failures.push(Failure {
            path: path.to_path_buf(),
            format: "elf64",
            kind: FailureKind::Invariant,
            message: format!("e_ident class={} != 2", elf.elf_hdr.e_ident[4]),
        });
        return;
    }

    let phdr_bytes = elf.elf_hdr.e_phnum as usize * elf.elf_hdr.e_phentsize as usize;
    let shdr_bytes = elf.elf_hdr.e_shnum as usize * elf.elf_hdr.e_shentsize as usize;
    let mut bad: Option<String> = None;
    if phdr_bytes > raw.len() {
        bad = Some(format!(
            "phdr table bytes {} > raw.len() {}",
            phdr_bytes,
            raw.len()
        ));
    } else if shdr_bytes > raw.len() {
        bad = Some(format!(
            "shdr table bytes {} > raw.len() {}",
            shdr_bytes,
            raw.len()
        ));
    }
    if let Some(msg) = bad {
        counts.malformed += 1;
        failures.push(Failure {
            path: path.to_path_buf(),
            format: "elf64",
            kind: FailureKind::Invariant,
            message: msg,
        });
        return;
    }

    if elf.elf_phdr.len() != elf.elf_hdr.e_phnum as usize {
        counts.malformed += 1;
        failures.push(Failure {
            path: path.to_path_buf(),
            format: "elf64",
            kind: FailureKind::Invariant,
            message: format!(
                "phdr count mismatch: parsed={} e_phnum={}",
                elf.elf_phdr.len(),
                elf.elf_hdr.e_phnum
            ),
        });
        return;
    }
    if elf.elf_shdr.len() != elf.elf_hdr.e_shnum as usize {
        counts.malformed += 1;
        failures.push(Failure {
            path: path.to_path_buf(),
            format: "elf64",
            kind: FailureKind::Invariant,
            message: format!(
                "shdr count mismatch: parsed={} e_shnum={}",
                elf.elf_shdr.len(),
                elf.elf_hdr.e_shnum
            ),
        });
        return;
    }

    counts.parsed_elf64 += 1;
}

fn process_file(
    path: &Path,
    cfg: &Config,
    failures: &mut Vec<Failure>,
    counts: &mut Counts,
) -> bool {
    let metadata = match std::fs::metadata(path) {
        Ok(m) => m,
        Err(_) => {
            counts.skipped_unreadable += 1;
            failures.push(Failure {
                path: path.to_path_buf(),
                format: "io",
                kind: FailureKind::Io,
                message: "cannot stat".into(),
            });
            return false;
        }
    };
    if metadata.len() > cfg.max_file_bytes {
        counts.skipped_too_large += 1;
        return false;
    }

    let prefix = match read_prefix(path, DETECT_PREFIX_LEN) {
        Some(b) if !b.is_empty() => b,
        _ => {
            counts.skipped_unreadable += 1;
            return false;
        }
    };

    let format = match detect_format(&prefix) {
        Some(f) => f,
        None => return false,
    };

    let raw = match std::fs::read(path) {
        Ok(b) => b,
        Err(_) => {
            counts.skipped_unreadable += 1;
            failures.push(Failure {
                path: path.to_path_buf(),
                format: "io",
                kind: FailureKind::Io,
                message: "cannot read full file".into(),
            });
            return false;
        }
    };

    match format {
        Format::Pe => handle_pe(path, &raw, failures, counts),
        Format::Elf { class } => {
            counts.candidate_elf += 1;
            match class {
                1 => handle_elf32(path, &raw, failures, counts),
                2 => handle_elf64(path, &raw, failures, counts),
                _ => {
                    counts.unknown_class += 1;
                    failures.push(Failure {
                        path: path.to_path_buf(),
                        format: "elf",
                        kind: FailureKind::Unsupported,
                        message: format!("unknown ELF class {class}"),
                    });
                }
            }
        }
    }
    true
}

fn apply_policy(cfg: &Config, counts: &Counts, failures: &[Failure]) {
    let hard_fail: Vec<&Failure> = failures
        .iter()
        .filter(|f| {
            matches!(f.kind, FailureKind::Panic)
                || (!cfg.allow_malformed
                    && matches!(
                        f.kind,
                        FailureKind::Invariant | FailureKind::ParseError
                    ))
        })
        .collect();

    let summary = format!(
        "dirs_scanned={} files_scanned={} candidate_pe={} candidate_elf={} \
         parsed_pe32={} parsed_pe64={} parsed_elf32={} parsed_elf64={} \
         skipped_too_large={} skipped_unreadable={} unknown_class={} \
         unknown_machine={} malformed={} panics={} hard_failures={}",
        counts.dirs_scanned,
        counts.files_scanned,
        counts.candidate_pe,
        counts.candidate_elf,
        counts.parsed_pe32,
        counts.parsed_pe64,
        counts.parsed_elf32,
        counts.parsed_elf64,
        counts.skipped_too_large,
        counts.skipped_unreadable,
        counts.unknown_class,
        counts.unknown_machine,
        counts.malformed,
        counts.panics,
        hard_fail.len()
    );
    println!("[bulk_parse] {summary}");

    let show = DETAILED_FAILURES_LIMIT.min(failures.len());
    for f in failures.iter().take(show) {
        println!(
            "[bulk_parse] failure format={} kind={} path={} msg={}",
            f.format,
            f.kind.label(),
            f.path.display(),
            f.message
        );
    }
    if failures.len() > show {
        println!(
            "[bulk_parse] ... {} more failure(s) omitted",
            failures.len() - show
        );
    }

    if !hard_fail.is_empty() {
        let first = hard_fail
            .iter()
            .take(5)
            .map(|f| format!("{} [{}]: {}", f.path.display(), f.kind.label(), f.message))
            .collect::<Vec<_>>()
            .join(" | ");
        if cfg.allow_malformed {
            panic!(
                "rs-header bulk parse: {} hard failure(s) (panics stay fatal even with RS_HEADER_ALLOW_MALFORMED=1). First: {}",
                hard_fail.len(),
                first
            );
        } else {
            panic!(
                "rs-header bulk parse: {} hard failure(s). First: {}",
                hard_fail.len(),
                first
            );
        }
    }
}

#[test]
#[ignore]
fn bulk_parse_pe_and_elf_corpus() {
    let cfg = match Config::from_env() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[bulk_parse] skipped: {e}");
            return;
        }
    };

    let (files, mut counts) = collect_files(&cfg);
    println!(
        "[bulk_parse] corpus_dirs={:?} files_to_scan={} max_file_mb={} allow_malformed={} limit={:?}",
        cfg.corpus_dirs, files.len(), cfg.max_file_bytes / (1024 * 1024), cfg.allow_malformed, cfg.limit
    );

    let mut failures: Vec<Failure> = Vec::new();

    let mut processed: usize = 0;
    for path in &files {
        let _ = process_file(path, &cfg, &mut failures, &mut counts);
        processed += 1;
        if let Some(limit) = cfg.limit {
            if processed >= limit {
                break;
            }
        }
    }

    apply_policy(&cfg, &counts, &failures);
}
