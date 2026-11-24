//! The opt-in guest filesystem for Linux userspace. File contents stay in emulator memory.
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use serde::{Deserialize, Serialize};

use crate::api::windows::helper;
use crate::emu::Emu;
use crate::syscall::linux::constants::errors::*;

mod mappings;
mod process;
pub use process::LinuxFileContext;
use process::{FileTable, PendingExec};
mod proc_maps;
mod syscalls;
pub(super) use syscalls::dispatch;
pub(crate) use syscalls::{close, open, read, write};

pub(super) fn external_descriptor(emu: &mut Emu, uri: Option<&str>) -> u64 {
    if let Some(files) = &mut emu.linux_files {
        let Some(fd) = helper::linux_descriptor(
            MAX_DESCRIPTORS as u32,
            |fd| files.open.contains_key(&fd),
            uri,
        ) else {
            return EMFILE;
        };
        files.external.insert(fd as u32);
        files.external_refs.insert(fd as u32, 1);
        return fd;
    }
    match uri {
        Some(uri) => helper::handler_create(uri),
        None => helper::socket_create(),
    }
}

pub(crate) fn open_needs_mode(flags: u32) -> bool {
    flags & O_CREAT != 0 || flags & O_TMPFILE == O_TMPFILE
}

const AUXV: &str = "/proc/self/auxv";
const INPUT: &str = "/proc/self/exe";
const FD_PREFIX: &str = "/proc/self/fd/";
const RETAINED_ELFS: usize = 8;
const RETAINED_ELF_BYTES: usize = 16 * 1024 * 1024;
const MAX_NODES: usize = 4096;
const MAX_DESCRIPTORS: usize = 1024;
const MAX_STORAGE: usize = 128 * 1024 * 1024;
const MAX_IO: usize = 1024 * 1024;
pub(super) const MAX_PATH: usize = 4096;
const AT_FDCWD: i32 = -100;
const O_ACCMODE: u32 = 3;
const O_WRONLY: u32 = 1;
const O_RDWR: u32 = 2;
const O_CREAT: u32 = 0o100;
const O_EXCL: u32 = 0o200;
const O_TRUNC: u32 = 0o1000;
const O_APPEND: u32 = 0o2000;
const O_DIRECTORY: u32 = 0o200000;
const O_CLOEXEC: u32 = 0o2000000;
const O_TMPFILE: u32 = 0o20200000;
const S_IFDIR: u32 = 0o040000;
const S_IFLNK: u32 = 0o120000;
const S_IFMT: u32 = 0o170000;
const S_IFCHR: u32 = 0o020000;
const S_IFREG: u32 = 0o100000;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Node {
    name: String,
    mode: u32,
    bytes: Vec<u8>,
    output: bool,
    #[serde(default)]
    mapping_dirty: bool,
    #[serde(default)]
    page_tail: Vec<u8>,
}

impl Node {
    fn directory(&self) -> bool {
        self.mode & S_IFMT == S_IFDIR
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct OpenFile {
    inode: u64,
    offset: u64,
    flags: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinuxFiles {
    pub args: Vec<String>,
    pub env: Vec<String>,
    pid: u64,
    paths: BTreeMap<String, u64>,
    nodes: BTreeMap<u64, Node>,
    open: FileTable,
    // The legacy pipe, event and socket backings live only in the current host process.
    #[serde(skip)]
    external: BTreeSet<u32>,
    #[serde(skip)]
    external_refs: BTreeMap<u32, usize>,
    retired: VecDeque<(String, Vec<u8>)>,
    #[serde(skip)]
    pending_exec: Option<PendingExec>,
    observed_exec: Option<u64>,
    exec_handoff: bool,
    executable_inode: u64,
    saved_inodes: BTreeMap<u64, usize>,
    executable: String,
    cwd: String,
    next_inode: u64,
    allocated: usize,
    #[serde(default)]
    mapping_stores: BTreeMap<u64, u64>,
}

impl Emu {
    /// Linux exposes the initial auxiliary vector even after userspace reuses its stack.
    pub(crate) fn save_linux_auxv(&mut self, address: u64, len: usize) {
        let Some(files) = &mut self.linux_files else {
            return;
        };
        let Some(bytes) = self.maps.read_bytes_option(address, len) else {
            return;
        };
        if let Some(node) = files.paths.get(AUXV).and_then(|id| files.nodes.get_mut(id)) {
            node.bytes = bytes.into_owned();
        } else {
            files.add_node(
                AUXV.into(),
                S_IFREG | 0o400,
                bytes.into_owned(),
                false,
                true,
            );
        }
    }

    /// Publish guest stores through MAP_SHARED before observing extracted files outside a syscall.
    pub fn sync_linux_files(&mut self) {
        if let Some(files) = &mut self.linux_files {
            files.sync_mappings(&self.maps);
        }
    }

    pub fn set_linux_files(&mut self, image: Vec<u8>, stdin: Vec<u8>) {
        self.linux_files = Some(LinuxFiles::new(
            self.processes.pid,
            &self.cfg.exe_name,
            image,
            stdin,
        ));
    }
}

impl LinuxFiles {
    pub(crate) fn copy_process_auxv(&mut self, other: &Self) {
        let path = "/proc/self/auxv";
        if let Some(node) = other.paths.get(path).and_then(|id| other.nodes.get(id))
            && let Some(dest) = self.paths.get(path).and_then(|id| self.nodes.get_mut(id))
        {
            dest.bytes.clone_from(&node.bytes);
        }
    }

    fn new(pid: u64, name: &str, image: Vec<u8>, stdin: Vec<u8>) -> Self {
        let name = name.rsplit(['/', '\\']).next().unwrap_or("input");
        let name = if matches!(name, "" | "." | "..") {
            "input"
        } else {
            name
        };
        let executable = format!("/app/{name}");
        let mut fs = Self {
            pid,
            args: Vec::new(),
            env: Vec::new(),
            paths: BTreeMap::new(),
            nodes: BTreeMap::new(),
            open: FileTable::default(),
            external: BTreeSet::new(),
            external_refs: BTreeMap::new(),
            retired: VecDeque::new(),
            pending_exec: None,
            observed_exec: None,
            exec_handoff: false,
            executable_inode: 0,
            saved_inodes: BTreeMap::new(),
            executable: executable.clone(),
            cwd: "/work".into(),
            next_inode: 1,
            allocated: 0,
            mapping_stores: BTreeMap::new(),
        };
        for path in [
            "/",
            "/app",
            "/work",
            "/proc",
            "/proc/self",
            "/proc/self/fd",
            "/dev",
            "/dev/shm",
            "/tmp",
        ] {
            fs.add_node(path.into(), S_IFDIR | 0o755, Vec::new(), false, true);
        }
        fs.add_node(
            "/dev/urandom".into(),
            S_IFCHR | 0o444,
            Vec::new(),
            false,
            true,
        );
        fs.add_node("/dev/null".into(), S_IFCHR | 0o666, Vec::new(), false, true);
        fs.executable_inode = fs.add_node(executable, S_IFREG | 0o555, image, false, true);
        let inode = fs.add_node("stdin".into(), S_IFREG | 0o444, stdin, false, false);
        fs.open.insert(
            0,
            OpenFile {
                inode,
                offset: 0,
                flags: 0,
            },
        );
        for (fd, name) in [(1, "stdout"), (2, "stderr")] {
            let inode = fs.add_node(name.into(), S_IFCHR | 0o222, Vec::new(), false, false);
            fs.open.insert(
                fd,
                OpenFile {
                    inode,
                    offset: 0,
                    flags: O_WRONLY,
                },
            );
        }
        for (fd, name) in [(0, "stdin"), (1, "stdout"), (2, "stderr")] {
            fs.add_node(
                format!("/dev/{name}"),
                S_IFLNK | 0o777,
                format!("{FD_PREFIX}{fd}").into_bytes(),
                false,
                true,
            );
        }
        fs
    }

    fn add_node(
        &mut self,
        name: String,
        mode: u32,
        bytes: Vec<u8>,
        output: bool,
        linked: bool,
    ) -> u64 {
        let id = self.next_inode;
        self.next_inode += 1;
        if linked {
            self.paths.insert(name.clone(), id);
        }
        self.nodes.insert(
            id,
            Node {
                name,
                mode,
                bytes,
                output,
                mapping_dirty: false,
                page_tail: Vec::new(),
            },
        );
        id
    }

    pub fn produced_files(&self) -> impl Iterator<Item = (&str, &[u8])> {
        self.nodes
            .values()
            .filter(|n| n.output && !n.directory() && !n.bytes.is_empty())
            .map(|n| (n.name.as_str(), n.bytes.as_slice()))
            .chain(
                self.retired
                    .iter()
                    .map(|(path, bytes)| (path.as_str(), bytes.as_slice())),
            )
    }

    /// True while the guest is stopped at an exec boundary that this architecture cannot enter.
    pub fn exec_handoff(&self) -> bool {
        self.exec_handoff
    }

    /// The last guest-produced ELF accepted by exec, including an explicit observer handoff.
    pub fn attempted_exec(&self) -> Option<(&str, &[u8])> {
        let node = self.nodes.get(&self.observed_exec?)?;
        Some((&node.name, &node.bytes))
    }

    pub fn read_at(&self, fd: u32, offset: u64, length: usize) -> Result<&[u8], u64> {
        let file = self.open.get(&fd).ok_or(EBADF)?;
        if file.flags & O_ACCMODE == O_WRONLY {
            return Err(EBADF);
        }
        let node = self.nodes.get(&file.inode).ok_or(EBADF)?;
        if node.directory() {
            return Err(EISDIR);
        }
        let start = usize::try_from(offset)
            .unwrap_or(usize::MAX)
            .min(node.bytes.len());
        Ok(&node.bytes[start..start.saturating_add(length).min(node.bytes.len())])
    }

    fn path(&self, dirfd: i32, path: &str) -> Result<String, u64> {
        if path.is_empty() {
            return Err(ENOENT);
        }
        let base = if path.starts_with('/') {
            "/"
        } else if dirfd == AT_FDCWD {
            &self.cwd
        } else {
            let node = self.node(dirfd as u32)?;
            if !node.directory() {
                return Err(ENOTDIR);
            }
            &node.name
        };
        let mut parts = Vec::new();
        for part in base.split('/').chain(path.split('/')) {
            match part {
                "" | "." => (),
                ".." => {
                    parts.pop();
                }
                name => parts.push(name),
            }
        }
        if let ["proc", process, ..] = parts.as_mut_slice()
            && process.parse::<u64>() == Ok(self.pid)
            && !process.starts_with(['0', '+'])
        {
            *process = "self";
        }
        Ok(format!("/{}", parts.join("/")))
    }

    fn lookup(&self, path: &str) -> Result<u64, u64> {
        let mut path = path.to_owned();
        for _ in 0..40 {
            let id = self.lookup_raw(&path)?;
            let node = self.nodes.get(&id).ok_or(ENOENT)?;
            if node.mode & S_IFMT != S_IFLNK {
                return Ok(id);
            }
            let target = std::str::from_utf8(&node.bytes).map_err(|_| EINVAL)?;
            let parent = path.rsplit_once('/').map_or("/", |(parent, _)| parent);
            path = if target.starts_with('/') {
                self.path(AT_FDCWD, target)?
            } else {
                self.path(AT_FDCWD, &format!("{parent}/{target}"))?
            };
        }
        Err(ELOOP)
    }

    fn lookup_raw(&self, path: &str) -> Result<u64, u64> {
        if let Some(fd) = path
            .strip_prefix(FD_PREFIX)
            .and_then(|s| s.parse::<u32>().ok())
        {
            return self.open.get(&fd).map(|f| f.inode).ok_or(ENOENT);
        }
        if path == INPUT {
            return Ok(self.executable_inode);
        }
        self.paths.get(path).copied().ok_or(ENOENT)
    }

    fn node(&self, fd: u32) -> Result<&Node, u64> {
        self.open
            .get(&fd)
            .and_then(|f| self.nodes.get(&f.inode))
            .ok_or(EBADF)
    }

    fn target(&self, path: &str) -> Result<Cow<'_, str>, u64> {
        if path == INPUT || path.starts_with(FD_PREFIX) {
            let inode = self.lookup_raw(path)?;
            let node = self.nodes.get(&inode).ok_or(ENOENT)?;
            if node.name.starts_with('/') && self.paths.get(&node.name) != Some(&inode) {
                return Ok(format!("{} (deleted)", node.name).into());
            }
            return Ok(node.name.as_str().into());
        }
        let id = self.lookup_raw(path)?;
        let node = self.nodes.get(&id).ok_or(ENOENT)?;
        if node.mode & S_IFMT == S_IFLNK {
            return std::str::from_utf8(&node.bytes)
                .map(Cow::Borrowed)
                .map_err(|_| EINVAL);
        }
        Err(EINVAL)
    }

    fn create(&mut self, path: String, mode: u32, linked: bool) -> Result<u64, u64> {
        if self.nodes.len() >= MAX_NODES {
            return Err(ENOSPC);
        }
        if linked {
            if self.lookup_raw(&path).is_ok() {
                return Err(EEXIST);
            }
            if !path.starts_with("/tmp/")
                && !path.starts_with("/dev/shm/")
                && !path.starts_with("/work/")
            {
                return Err(EROFS);
            }
            let parent = path.rsplit_once('/').map(|(p, _)| p).ok_or(ENOENT)?;
            let parent = self.lookup(parent)?;
            if !self.nodes.get(&parent).is_some_and(Node::directory) {
                return Err(ENOTDIR);
            }
        }
        Ok(self.add_node(path, mode, Vec::new(), true, linked))
    }

    fn proc_snapshot(&mut self, name: String, bytes: Vec<u8>, flags: u32) -> Result<u64, u64> {
        if flags & O_ACCMODE != 0 {
            return Err(EROFS);
        }
        if self.nodes.len() >= MAX_NODES || bytes.len() > MAX_STORAGE - self.allocated {
            return Err(ENOSPC);
        }
        let fd = self.next_fd()?;
        let len = bytes.len();
        let inode = self.add_node(name, S_IFREG | 0o444, bytes, false, false);
        self.allocated += len;
        self.open.insert(
            fd,
            OpenFile {
                inode,
                offset: 0,
                flags,
            },
        );
        Ok(fd as u64)
    }

    fn next_fd(&self) -> Result<u32, u64> {
        (0..MAX_DESCRIPTORS as u32)
            .find(|fd| {
                !self.open.contains_key(fd)
                    && (*fd < 3
                        || (!helper::handler_exist(*fd as u64)
                            && !helper::socket_exist(*fd as u64)))
            })
            .ok_or(EMFILE)
    }

    fn create_memfd(&mut self, label: String, flags: u32) -> Result<u64, u64> {
        if flags & !3 != 0 || label.len() > 249 {
            return Err(EINVAL);
        }
        let fd = self.next_fd()?;
        let inode = self.create(format!("memfd:{label}"), S_IFREG | 0o777, false)?;
        self.open.insert(
            fd,
            OpenFile {
                inode,
                offset: 0,
                flags: O_RDWR | if flags & 1 != 0 { O_CLOEXEC } else { 0 },
            },
        );
        Ok(fd as u64)
    }

    fn open_path(&mut self, path: String, flags: u32, mode: u32) -> Result<u64, u64> {
        let fd = self.next_fd()?;
        let tmp = flags & O_TMPFILE == O_TMPFILE;
        let inode = if tmp {
            if flags & O_ACCMODE == 0 {
                return Err(EINVAL);
            }
            let dir = self.lookup(&path)?;
            if !self.nodes.get(&dir).is_some_and(Node::directory) {
                return Err(ENOTDIR);
            }
            if path != "/tmp"
                && path != "/dev/shm"
                && !path.starts_with("/tmp/")
                && !path.starts_with("/dev/shm/")
            {
                return Err(EROFS);
            }
            self.create(
                format!("{path}/(anonymous)"),
                S_IFREG | (mode & 0o777),
                false,
            )?
        } else {
            match self.lookup(&path) {
                Ok(_) if flags & (O_CREAT | O_EXCL) == O_CREAT | O_EXCL => return Err(EEXIST),
                Ok(id) => id,
                Err(ENOENT) if flags & O_CREAT != 0 => {
                    self.create(path, S_IFREG | (mode & 0o777), true)?
                }
                Err(e) => return Err(e),
            }
        };
        let node = self.nodes.get(&inode).ok_or(ENOENT)?;
        if flags & O_DIRECTORY != 0 && !tmp && !node.directory() {
            return Err(ENOTDIR);
        }
        if flags & O_ACCMODE != 0 && node.directory() {
            return Err(EISDIR);
        }
        let writable_device = node.mode & S_IFMT == S_IFCHR && node.mode & 0o222 != 0;
        if flags & O_ACCMODE != 0 && !node.output && !writable_device {
            return Err(EROFS);
        }
        if flags & O_TRUNC != 0 && flags & O_ACCMODE != 0 {
            self.resize(inode, 0)?;
        }
        self.open.insert(
            fd,
            OpenFile {
                inode,
                offset: 0,
                flags,
            },
        );
        Ok(fd as u64)
    }

    fn resize(&mut self, inode: u64, size: u64) -> Result<(), u64> {
        let node = self.nodes.get_mut(&inode).ok_or(EBADF)?;
        if !node.output || node.directory() {
            return Err(EBADF);
        }
        let size = usize::try_from(size).map_err(|_| EFBIG)?;
        let used = self.allocated - node.bytes.len();
        if size > MAX_STORAGE - used {
            return Err(ENOSPC);
        }
        if size != node.bytes.len() {
            node.page_tail.clear();
        }
        node.bytes.resize(size, 0);
        node.mapping_dirty = self.mapping_stores.contains_key(&inode);
        self.allocated = used + size;
        Ok(())
    }

    fn truncate(&mut self, fd: u32, size: u64) -> Result<u64, u64> {
        let f = self.open.get(&fd).ok_or(EBADF)?;
        if f.flags & O_ACCMODE == 0 {
            return Err(EINVAL);
        }
        self.resize(f.inode, size)?;
        Ok(0)
    }

    fn write(&mut self, fd: u32, bytes: &[u8]) -> Result<u64, u64> {
        let f = *self.open.get(&fd).ok_or(EBADF)?;
        if f.flags & O_ACCMODE == 0 {
            return Err(EBADF);
        }
        let len = self.node(fd)?.bytes.len();
        let offset = if f.flags & O_APPEND != 0 {
            len as u64
        } else {
            f.offset
        };
        let end = offset.checked_add(bytes.len() as u64).ok_or(EFBIG)?;
        if !bytes.is_empty() && end > len as u64 {
            self.resize(f.inode, end)?;
        }
        if !bytes.is_empty() {
            let node = self.nodes.get_mut(&f.inode).ok_or(EBADF)?;
            node.bytes[offset as usize..end as usize].copy_from_slice(bytes);
            node.mapping_dirty = self.mapping_stores.contains_key(&f.inode);
        }
        self.open.get_mut(&fd).ok_or(EBADF)?.offset = end;
        Ok(bytes.len() as u64)
    }

    fn seek(&mut self, fd: u32, offset: i64, whence: u64) -> Result<u64, u64> {
        let node = self.node(fd)?;
        if matches!(node.name.as_str(), "stdin" | "stdout" | "stderr") {
            return Err(ESPIPE);
        }
        let end = (!node.directory()).then_some(node.bytes.len() as u64);
        let f = self.open.get_mut(&fd).ok_or(EBADF)?;
        let base = match whence {
            0 => 0,
            1 => f.offset,
            2 => end.ok_or(EINVAL)?,
            _ => return Err(EINVAL),
        };
        f.offset = base
            .checked_add_signed(offset)
            .filter(|off| *off <= i64::MAX as u64)
            .ok_or(EINVAL)?;
        Ok(f.offset)
    }

    fn release(&mut self, inode: u64) {
        if !self.mapping_stores.contains_key(&inode)
            && self.executable_inode != inode
            && self.observed_exec != Some(inode)
            && !self.saved_inodes.contains_key(&inode)
            && !self.paths.values().any(|&id| id == inode)
            && !self.open.values().any(|f| f.inode == inode)
            && let Some(node) = self.nodes.remove(&inode)
            && (node.output || node.name == "/proc/self/maps")
        {
            self.allocated -= node.bytes.len();
            if node.output
                && node.bytes.starts_with(b"\x7fELF")
                && node.bytes.len() <= RETAINED_ELF_BYTES
            {
                let mut size = node.bytes.len()
                    + self
                        .retired
                        .iter()
                        .map(|(_, bytes)| bytes.len())
                        .sum::<usize>();
                while self.retired.len() >= RETAINED_ELFS || size > RETAINED_ELF_BYTES {
                    if let Some((_, bytes)) = self.retired.pop_front() {
                        size -= bytes.len();
                    } else {
                        break;
                    }
                }
                self.retired.push_back((node.name, node.bytes));
            }
        }
    }

    fn close(&mut self, fd: u32) -> Result<u64, u64> {
        let f = self.open.remove(&fd).ok_or(EBADF)?;
        self.release(f.inode);
        Ok(0)
    }

    fn unlink(&mut self, path: &str, directory: bool) -> Result<u64, u64> {
        if path.starts_with("/proc/") {
            return Err(EROFS);
        }
        let inode = self.lookup_raw(path)?;
        let node = self.nodes.get(&inode).ok_or(ENOENT)?;
        if !node.output {
            return Err(EROFS);
        }
        if node.directory() != directory {
            return Err(if directory { ENOTDIR } else { EISDIR });
        }
        if directory
            && self.paths.keys().any(|p| {
                p.strip_prefix(path)
                    .is_some_and(|rest| rest.starts_with('/'))
            })
        {
            return Err(ENOTEMPTY);
        }
        self.paths.remove(path);
        self.release(inode);
        Ok(0)
    }
}
