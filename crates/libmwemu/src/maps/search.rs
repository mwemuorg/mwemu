use super::Maps;
use crate::maps::mem64::Mem64;

impl Maps {
    pub fn search_string(&self, kw: &str, map_name: &str) -> Option<Vec<u64>> {
        /*
        TODO: We can use AVX2 instructions to speed up the comparision but I don't know how to do it in rust
        reference: https://github.com/0x1F9F1/mem/blob/master/include/mem/simd_scanner.h
        maybe using https://github.com/greaka/patterns
         */
        let map = self.get_map_by_name(map_name);
        if map.is_none() {
            log::trace!("map not found");
            return None;
        }
        let mem = map.unwrap();
        let bkw = kw.as_bytes();

        let mut found: Vec<u64> = Vec::new();
        for addr in mem.get_base()..mem.get_bottom() {
            let mut c = 0;

            for (i, bkwi) in bkw.iter().enumerate() {
                let b = mem.read_byte(addr + (i as u64));

                if b == *bkwi {
                    c += 1;
                } else {
                    break;
                }
            }

            if c == kw.len() {
                found.push(addr);
            }
        }

        if !found.is_empty() {
            Some(found)
        } else {
            log::trace!("map not found");
            None
        }
    }

    pub fn write_spaced_bytes(&mut self, addr: u64, sbs: &str) -> bool {
        let mut waddr = addr;
        let bs: Vec<&str> = sbs.split(' ').collect();
        for bsi in bs.iter() {
            let b = u8::from_str_radix(bsi, 16).expect("bad num conversion");
            if !self.write_byte(waddr, b) {
                return false;
            }
            waddr += 1;
        }
        true
    }

    pub fn spaced_bytes_to_bytes(&self, sbs: &str) -> Vec<u8> {
        let bs: Vec<&str> = sbs.split(' ').collect();
        let mut bytes: Vec<u8> = Vec::new();
        for bsi in bs.iter() {
            let b = match u8::from_str_radix(bsi, 16) {
                Ok(b) => b,
                Err(_) => {
                    log::trace!("bad hex bytes");
                    return bytes;
                }
            };
            bytes.push(b);
        }
        bytes
    }

    #[inline]
    fn is_pattern_match_at(memory: &Mem64, address: u64, pattern: &Vec<u8>) -> bool {
        for (i, &pattern_byte) in pattern.iter().enumerate() {
            let current_addr = address + (i as u64);

            // If we reach the end of the memory region, the pattern doesn't match
            if current_addr >= memory.get_bottom() {
                return false;
            }

            // If the byte doesn't match, the pattern doesn't match
            if memory.read_byte(current_addr) != pattern_byte {
                return false;
            }
        }

        // All bytes matched
        true
    }

    // search only one occurence from specific address
    pub fn search_spaced_bytes_from(&self, sbs: &str, saddr: u64) -> u64 {
        let byte_pattern = self.spaced_bytes_to_bytes(sbs);

        // Find the memory region containing the start address
        for (_, memory) in self.mem_slab.iter() {
            // Skip memory regions that don't contain the start address
            if saddr < memory.get_base() || saddr >= memory.get_bottom() {
                continue;
            }

            // Search backwards from start_address to base address
            for current_addr in memory.get_base()..=saddr {
                if Maps::is_pattern_match_at(memory, current_addr, &byte_pattern) {
                    return current_addr;
                }
            }

            // If we searched the entire memory region and didn't find a match, return 0
            return 0;
        }

        // No matching memory region found
        0
    }

    // search only one occurence from specific address backward
    pub fn search_spaced_bytes_from_bw(&self, spaced_bytes: &str, start_address: u64) -> u64 {
        let byte_pattern = self.spaced_bytes_to_bytes(spaced_bytes);

        // Find the memory region containing the start address
        for (_, memory) in self.mem_slab.iter() {
            // Skip memory regions that don't contain the start address
            if start_address < memory.get_base() || start_address >= memory.get_bottom() {
                continue;
            }

            // Search backwards from start_address to base address
            for current_addr in (memory.get_base()..=start_address).rev() {
                if Maps::is_pattern_match_at(memory, current_addr, &byte_pattern) {
                    return current_addr;
                }
            }

            // If we searched the entire memory region and didn't find a match, return 0
            return 0;
        }

        // No matching memory region found
        0
    }

    pub fn search_spaced_bytes(&self, sbs: &str, map_name: &str) -> Vec<u64> {
        let bytes = self.spaced_bytes_to_bytes(sbs);
        self.search_bytes(bytes, map_name)
    }

    pub fn search_spaced_bytes_in_all(&self, sbs: &str) -> Vec<u64> {
        let bytes = self.spaced_bytes_to_bytes(sbs);
        let mut found: Vec<u64> = Vec::new();

        for (_, mem) in self.mem_slab.iter() {
            for addr in mem.get_base()..mem.get_bottom() {
                if addr < 0x70000000 {
                    let mut c = 0;
                    for (i, bi) in bytes.iter().enumerate() {
                        let addri = addr + (i as u64);
                        if !mem.inside(addri) {
                            break;
                        }

                        let b = mem.read_byte(addri);
                        if b == *bi {
                            c += 1;
                        } else {
                            break;
                        }
                    }

                    if c == bytes.len() {
                        found.push(addr);
                    }
                }
            }
        }

        found
    }

    //TODO: return a list with matches.
    pub fn search_string_in_all(&self, kw: String) {
        let mut found = false;
        for (_, mem) in self.mem_slab.iter() {
            if mem.get_base() >= 0x7000000 {
                continue;
            }

            let results = match self.search_string(&kw, mem.get_name()) {
                Some(v) => v,
                None => {
                    continue;
                }
            };

            for addr in results.iter() {
                if self.is_64bits {
                    log::trace!("found at 0x{:x} '{}'", addr, self.read_string(*addr));
                } else {
                    log::trace!(
                        "found at 0x{:x} '{}'",
                        *addr as u32,
                        self.read_string(*addr)
                    );
                }
                found = true;
            }
        }

        if !found {
            log::trace!("not found.");
        }
    }

    pub fn search_bytes(&self, bkw: Vec<u8>, map_name: &str) -> Vec<u64> {
        let mut addrs: Vec<u64> = Vec::new();

        for (_, mem) in self.mem_slab.iter() {
            if mem.get_name() == map_name {
                for addr in mem.get_base()..mem.get_bottom() {
                    let mut c = 0;

                    for (i, bkwn) in bkw.iter().enumerate() {
                        if addr + i as u64 >= mem.get_bottom() {
                            break;
                        }
                        let b = mem.read_byte(addr + (i as u64));
                        if b == *bkwn {
                            c += 1;
                        } else {
                            break;
                        }
                    }

                    if c == bkw.len() {
                        addrs.push(addr);
                    }
                }

                return addrs;
            }
        }
        addrs
    }
}
