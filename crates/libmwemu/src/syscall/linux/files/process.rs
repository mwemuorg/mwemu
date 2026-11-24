use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::syscall::linux::constants::errors::{
    EACCES, EBADF, EEXIST, EINVAL, EMFILE, ENOENT, ENOEXEC, ENOSPC, ENOTDIR,
};
use crate::syscall::linux::io;

use super::{
    AT_FDCWD, LinuxFiles, MAX_DESCRIPTORS, MAX_NODES, MAX_PATH, MAX_STORAGE, O_CLOEXEC, OpenFile,
    S_IFDIR, S_IFMT, S_IFREG,
};

#[derive(Clone, Debug)]
pub(super) struct PendingExec {
    pub inode: u64,
    pub filename: String,
    pub argv: Vec<String>,
    pub envp: Vec<String>,
}

type ExecImage<'a> = (&'a str, &'a [u8], &'a [String], &'a [String]);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct Descriptor {
    description: u64,
    close_on_exec: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub(super) struct FileTable {
    descriptors: BTreeMap<u32, Descriptor>,
    // fork and dup share the offset and file status flags of an open description.
    descriptions: BTreeMap<u64, (OpenFile, usize)>,
    next: u64,
}

impl FileTable {
    pub(super) fn get(&self, fd: &u32) -> Option<&OpenFile> {
        self.descriptions
            .get(&self.descriptors.get(fd)?.description)
            .map(|(file, _)| file)
    }

    pub(super) fn get_mut(&mut self, fd: &u32) -> Option<&mut OpenFile> {
        self.descriptions
            .get_mut(&self.descriptors.get(fd)?.description)
            .map(|(file, _)| file)
    }

    pub(super) fn contains_key(&self, fd: &u32) -> bool {
        self.descriptors.contains_key(fd)
    }

    pub(super) fn values(&self) -> impl Iterator<Item = &OpenFile> {
        self.descriptions.values().map(|(file, _)| file)
    }

    pub(super) fn insert(&mut self, fd: u32, mut file: OpenFile) {
        self.next += 1;
        let close_on_exec = file.flags & O_CLOEXEC != 0;
        file.flags &= !O_CLOEXEC;
        self.descriptors.insert(
            fd,
            Descriptor {
                description: self.next,
                close_on_exec,
            },
        );
        self.descriptions.insert(self.next, (file, 1));
    }

    pub(super) fn remove(&mut self, fd: &u32) -> Option<OpenFile> {
        let descriptor = self.descriptors.remove(fd)?;
        self.release(descriptor)
    }

    fn release(&mut self, descriptor: Descriptor) -> Option<OpenFile> {
        let (file, links) = self.descriptions.get_mut(&descriptor.description)?;
        let old = *file;
        *links -= 1;
        if *links == 0 {
            self.descriptions.remove(&descriptor.description);
        }
        Some(old)
    }

    pub(super) fn close_on_exec(&self, fd: u32) -> Result<bool, u64> {
        self.descriptors
            .get(&fd)
            .map(|f| f.close_on_exec)
            .ok_or(EBADF)
    }

    pub(super) fn set_close_on_exec(&mut self, fd: u32, enabled: bool) -> Result<u64, u64> {
        self.descriptors.get_mut(&fd).ok_or(EBADF)?.close_on_exec = enabled;
        Ok(0)
    }
}

#[derive(Debug)]
pub struct LinuxFileContext {
    external: BTreeSet<u32>,
    descriptors: BTreeMap<u32, Descriptor>,
    cwd: String,
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    executable_inode: u64,
    mappings: BTreeMap<u64, u64>,
    auxv: Option<Vec<u8>>,
}

impl LinuxFiles {
    /// Install an explicitly supplied read-only guest executable. All paths name guest files.
    pub fn mount_executable(&mut self, path: &str, image: &[u8]) -> Result<(), u64> {
        if !path.starts_with('/')
            || path.ends_with('/')
            || path.len() > MAX_PATH
            || path.split('/').any(|part| part == "." || part == "..")
        {
            return Err(EINVAL);
        }
        if !image.starts_with(b"\x7fELF") {
            return Err(ENOEXEC);
        }
        let path = self.path(AT_FDCWD, path)?;
        if self.paths.contains_key(&path) {
            return Err(EEXIST);
        }
        if image.len() > MAX_STORAGE.saturating_sub(self.allocated) {
            return Err(ENOSPC);
        }
        let mut missing = Vec::new();
        for (index, _) in path.match_indices('/').skip(1) {
            let parent = &path[..index];
            if let Some(node) = self.paths.get(parent).and_then(|id| self.nodes.get(id)) {
                if !node.directory() {
                    return Err(ENOTDIR);
                }
            } else {
                missing.push(parent.to_owned());
            }
        }
        if self.nodes.len() + missing.len() + 1 > MAX_NODES {
            return Err(ENOSPC);
        }
        for dir in missing {
            self.add_node(dir, S_IFDIR | 0o555, Vec::new(), false, true);
        }
        self.add_node(path, S_IFREG | 0o555, image.to_vec(), false, true);
        self.allocated += image.len();
        Ok(())
    }

    pub(crate) fn executable_bytes(&self, path: &str) -> Result<&[u8], u64> {
        let inode = self.lookup(&self.path(AT_FDCWD, path)?)?;
        let node = self.nodes.get(&inode).ok_or(ENOENT)?;
        if node.mode & S_IFMT != S_IFREG || node.mode & 0o111 == 0 {
            return Err(EACCES);
        }
        Ok(&node.bytes)
    }

    pub(crate) fn exec_request(&self) -> Option<ExecImage<'_>> {
        let request = self.pending_exec.as_ref()?;
        let node = self.nodes.get(&request.inode)?;
        Some((&request.filename, &node.bytes, &request.argv, &request.envp))
    }

    pub(super) fn record_exec(&mut self, inode: u64) {
        if self.nodes.get(&inode).is_some_and(|node| node.output)
            && let Some(old) = self.observed_exec.replace(inode)
        {
            self.release(old);
        }
    }

    pub fn begin_child(&mut self) -> LinuxFileContext {
        let descriptors = self.open.descriptors.clone();
        for &inode in self.mapping_stores.keys().chain([&self.executable_inode]) {
            *self.saved_inodes.entry(inode).or_default() += 1;
        }
        for fd in &self.external {
            if let Some(count) = self.external_refs.get_mut(fd) {
                *count += 1;
            }
        }
        for fd in descriptors.values() {
            if let Some((_, links)) = self.open.descriptions.get_mut(&fd.description) {
                *links += 1;
            }
        }
        LinuxFileContext {
            external: self.external.clone(),
            descriptors,
            cwd: self.cwd.clone(),
            executable: self.executable.clone(),
            args: self.args.clone(),
            env: self.env.clone(),
            executable_inode: self.executable_inode,
            mappings: self.mapping_stores.clone(),
            auxv: self
                .paths
                .get(super::AUXV)
                .and_then(|id| self.nodes.get(id))
                .map(|node| node.bytes.clone()),
        }
    }

    fn unpin_context(&mut self, saved: &LinuxFileContext) {
        for &inode in saved.mappings.keys().chain([&saved.executable_inode]) {
            if let Some(count) = self.saved_inodes.get_mut(&inode) {
                *count -= 1;
                if *count == 0 {
                    self.saved_inodes.remove(&inode);
                    self.release(inode);
                }
            }
        }
    }

    pub(crate) fn drop_context(&mut self, saved: LinuxFileContext) {
        for descriptor in saved.descriptors.values() {
            if let Some(file) = self.open.release(*descriptor) {
                self.release(file.inode);
            }
        }
        for &fd in &saved.external {
            self.drop_external_ref(fd);
        }
        self.unpin_context(&saved);
    }

    pub fn restore_parent(&mut self, saved: LinuxFileContext) {
        let fds: Vec<_> = self.open.descriptors.keys().copied().collect();
        let old_image = std::mem::replace(&mut self.executable_inode, saved.executable_inode);
        let old_mappings = std::mem::replace(&mut self.mapping_stores, saved.mappings.clone());
        self.unpin_context(&saved);
        self.release(old_image);
        for &inode in old_mappings.keys() {
            self.release(inode);
        }
        self.pending_exec = None;
        self.exec_handoff = false;
        self.env = saved.env;
        self.cwd = saved.cwd;
        self.executable = saved.executable;
        self.args = saved.args;
        for inode in self.mapping_stores.keys() {
            if let Some(node) = self.nodes.get_mut(inode) {
                node.mapping_dirty = true;
            }
        }
        if let Some(inode) = self.paths.get(super::AUXV).copied() {
            if let Some(bytes) = saved.auxv {
                if let Some(node) = self.nodes.get_mut(&inode) {
                    node.bytes = bytes;
                }
            } else {
                self.paths.remove(super::AUXV);
                self.release(inode);
            }
        }
        for fd in fds {
            let _ = self.close(fd);
        }
        self.open.descriptors = saved.descriptors;
        let fds: Vec<_> = self.external.iter().copied().collect();
        for fd in fds {
            self.close_external(fd);
        }
        self.external = saved.external;
    }

    pub fn enter_exec(&mut self) -> Result<(), u64> {
        let request = self.pending_exec.take().ok_or(ENOEXEC)?;
        let node = self.nodes.get(&request.inode).ok_or(ENOENT)?;
        self.executable.clone_from(&node.name);
        self.env = request.envp;
        self.args = request.argv.into_iter().skip(1).collect();
        let old_image = std::mem::replace(&mut self.executable_inode, request.inode);
        self.record_exec(request.inode);
        self.exec_handoff = false;
        self.release(old_image);
        for (inode, _) in std::mem::take(&mut self.mapping_stores) {
            self.release(inode);
        }
        let fds: Vec<_> = self
            .open
            .descriptors
            .iter()
            .filter_map(|(&fd, d)| d.close_on_exec.then_some(fd))
            .collect();
        for fd in fds {
            self.close(fd)?;
        }
        Ok(())
    }

    pub(crate) fn close_external(&mut self, fd: u32) -> u64 {
        if !self.external.remove(&fd) {
            return EBADF;
        }
        self.drop_external_ref(fd)
    }

    fn drop_external_ref(&mut self, fd: u32) -> u64 {
        let Some(count) = self.external_refs.get_mut(&fd) else {
            return EBADF;
        };
        *count -= 1;
        if *count != 0 {
            return 0;
        }
        self.external_refs.remove(&fd);
        io::close_fd(fd)
    }

    pub(super) fn duplicate(
        &mut self,
        source: u32,
        target: Option<u32>,
        cloexec: bool,
    ) -> Result<u64, u64> {
        let descriptor = *self.open.descriptors.get(&source).ok_or(EBADF)?;
        let target = match target {
            Some(target) if target == source => return Ok(target as u64),
            Some(target) if target >= MAX_DESCRIPTORS as u32 => return Err(EBADF),
            Some(target) => target,
            None => self.next_fd()?,
        };
        if self.open.descriptors.len() >= MAX_DESCRIPTORS && !self.open.contains_key(&target) {
            return Err(EMFILE);
        }
        if self.open.contains_key(&target) {
            self.close(target)?;
        } else if self.external.contains(&target) {
            self.close_external(target);
        }
        let (_, links) = self
            .open
            .descriptions
            .get_mut(&descriptor.description)
            .ok_or(EBADF)?;
        *links += 1;
        self.open.descriptors.insert(
            target,
            Descriptor {
                description: descriptor.description,
                close_on_exec: cloexec,
            },
        );
        Ok(target as u64)
    }
}
