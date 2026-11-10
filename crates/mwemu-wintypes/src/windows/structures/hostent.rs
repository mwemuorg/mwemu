use crate::maps::Maps;

pub struct Hostent {
    pub hname: u64,
    pub alias_list: u64,
    pub addr_type: u16,
    pub length: u16,
    pub addr_list: u64,
    // (gdb) 0x7ffff7fa0b60 -> 0x5555555595d0 -> 0x5555555595cc -> IP
}

impl Default for Hostent {
    fn default() -> Self {
        Self::new()
    }
}

impl Hostent {
    pub fn new() -> Hostent {
        Hostent {
            hname: 0,
            alias_list: 0,
            addr_type: 0,
            length: 4,
            addr_list: 0,
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.hname);
        maps.write_qword(addr + 8, self.alias_list);
        maps.write_word(addr + 16, self.addr_type);
        maps.write_word(addr + 20, self.length);
        maps.write_qword(addr + 24, self.addr_list);
    }

    pub fn size() -> usize {
        32
    }
}
