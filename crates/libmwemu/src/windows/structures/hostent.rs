use crate::maps::Maps;

pub struct Hostent {
    pub hname: u64,
    pub alias_list: u64,
    pub addr_type: u16,
    pub length: u16,
    pub addr_list: u64,
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
        if maps.is_64bits {
            maps.write_qword(addr, self.hname);
            maps.write_qword(addr + 8, self.alias_list);
            maps.write_word(addr + 16, self.addr_type);
            maps.write_word(addr + 18, self.length);
            maps.write_qword(addr + 24, self.addr_list);
        } else {
            maps.write_dword(addr, self.hname as u32);
            maps.write_dword(addr + 4, self.alias_list as u32);
            maps.write_word(addr + 8, self.addr_type);
            maps.write_word(addr + 10, self.length);
            maps.write_dword(addr + 12, self.addr_list as u32);
        }
    }

    pub fn size() -> usize {
        32
    }
}
