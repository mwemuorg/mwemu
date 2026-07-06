use crate::maps::Maps;

#[derive(Debug)]
pub struct Stat {
    // used by fstat syscall
    pub dev: u64,
    pub ino: u64,
    pub nlink: u64,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub pad0: u32,
    pub rdev: u64,
    pub size: i64,
    pub blksize: i64,
    pub blocks: i64,
    pub atime_sec: u64,
    pub atime_nsec: u64,
    pub mtime_sec: u64,
    pub mtime_nsec: u64,
    pub ctime_sec: u64,
    pub ctime_nsec: u64,
    pub reserved: [i64; 3],
}

impl Stat {
    pub fn fake() -> Stat {
        Stat {
            dev: 64769,
            ino: 41946037,
            nlink: 1,
            mode: 33188,
            uid: 0,
            gid: 0,
            pad0: 0,
            rdev: 0,
            size: 2794,
            blksize: 4096,
            blocks: 8,
            atime_sec: 1692634621,
            atime_nsec: 419117625,
            mtime_sec: 1690443336,
            mtime_nsec: 991482376,
            ctime_sec: 1690443336,
            ctime_nsec: 995482376,
            reserved: [0; 3],
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.dev);
        maps.write_qword(addr + 8, self.ino);
        maps.write_qword(addr + 16, self.nlink);
        maps.write_dword(addr + 24, self.mode);
        maps.write_dword(addr + 28, self.uid);
        maps.write_dword(addr + 32, self.gid);
        maps.write_dword(addr + 36, self.pad0);
        maps.write_qword(addr + 40, self.rdev);
        maps.write_qword(addr + 48, self.size as u64);
        maps.write_qword(addr + 56, self.blksize as u64);
        maps.write_qword(addr + 64, self.blocks as u64);
        maps.write_qword(addr + 72, self.atime_sec);
        maps.write_qword(addr + 80, self.atime_nsec);
        maps.write_qword(addr + 88, self.mtime_sec);
        maps.write_qword(addr + 96, self.mtime_nsec);
        maps.write_qword(addr + 104, self.ctime_sec);
        maps.write_qword(addr + 112, self.ctime_nsec);
        maps.write_qword(addr + 120, self.reserved[0] as u64);
        maps.write_qword(addr + 128, self.reserved[1] as u64);
        maps.write_qword(addr + 136, self.reserved[2] as u64);
    }

    pub fn size() -> usize {
        144
    }
}
