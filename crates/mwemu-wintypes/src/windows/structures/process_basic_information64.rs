use crate::maps::Maps;

pub struct ProcessBasicInformation64 {
    pub Reserved1: u64,
    pub PebBaseAddress: u64,
    pub Reserved2: [u64; 2],
    pub UniqueProcessId: u64,
    pub Reserved3: u64,
}

impl ProcessBasicInformation64 {
    pub fn size() -> u64 {
        0x30
    }

    pub fn load(addr: u64, maps: &Maps) -> ProcessBasicInformation64 {
        ProcessBasicInformation64 {
            Reserved1: maps.read_qword(addr).unwrap_or(0),
            PebBaseAddress: maps.read_qword(addr + 8).unwrap_or(0),
            Reserved2: [
                maps.read_qword(addr + 16).unwrap_or(0),
                maps.read_qword(addr + 24).unwrap_or(0),
            ],
            UniqueProcessId: maps.read_qword(addr + 32).unwrap_or(0),
            Reserved3: maps.read_qword(addr + 40).unwrap_or(0),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.Reserved1);
        maps.write_qword(addr + 8, self.PebBaseAddress);
        maps.write_qword(addr + 16, self.Reserved2[0]);
        maps.write_qword(addr + 24, self.Reserved2[1]);
        maps.write_qword(addr + 32, self.UniqueProcessId);
        maps.write_qword(addr + 40, self.Reserved3);
    }
}
