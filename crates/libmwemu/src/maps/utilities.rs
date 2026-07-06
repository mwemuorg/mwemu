use super::Maps;

impl Maps {
    #[inline(always)]
    pub fn memset(&mut self, addr: u64, b: u8, amount: usize) {
        // Bulk-write when the whole range fits in one writable map. Falls back
        // to byte-by-byte only for boundary-crossing cases, and stops on the
        // first failed byte instead of spamming the log with one warning per
        // out-of-range byte.
        if amount == 0 {
            return;
        }
        let end_addr = addr + amount as u64 - 1;
        if let Some(mem) = self.get_mem_by_addr_mut(addr) {
            if mem.inside(end_addr) && mem.can_write() {
                let buf = vec![b; amount];
                mem.write_bytes(addr, &buf);
                return;
            }
        }
        for i in 0..amount {
            if !self.write_byte(addr + i as u64, b) {
                log::warn!(
                    "memset truncated at 0x{:x} ({} of {} bytes written; destination not fully writable). caller request was addr=0x{:x} amount={}",
                    addr + i as u64,
                    i,
                    amount,
                    addr,
                    amount,
                );
                // Print a small Rust backtrace so we can identify the caller cheaply.
                let bt = std::backtrace::Backtrace::force_capture();
                log::warn!("backtrace:\n{}", bt);
                return;
            }
        }
    }

    pub fn memcpy(&mut self, to: u64, from: u64, size: usize) -> bool {
        let b = match self.read_bytes_option(from, size) {
            None => return false,
            Some(b) => b.to_vec(),
        };
        self.write_bytes(to, &b);
        true
    }

    pub fn sizeof_wide(&self, unicode_str_ptr: u64) -> usize {
        const MAX_STR_LEN: usize = 1_000_000;
        let mut counter: usize = 0;

        for i in (0..MAX_STR_LEN).step_by(2) {
            let b = match self.read_word(unicode_str_ptr + i as u64) {
                Some(w) => w,
                None => return counter,
            };
            if b == 0 {
                return counter;
            }
            counter += 1;
        }

        0
    }

    pub fn write_string(&mut self, to: u64, from: &str) {
        //log::debug!("write_string 0x{:x}: `{}`", to, from);
        let bs: Vec<u8> = from.bytes().collect();

        self.write_bytes(to, &bs);
        self.write_byte(to + bs.len() as u64, 0x00);
    }

    pub fn write_wide_string(&mut self, to: u64, from: &str) {
        let Some(mem) = self.get_mem_by_addr_mut(to) else {
            log::warn!("Cannot write wide string: no map at 0x{:x}", to);
            return;
        };
        if !mem.can_write() {
            log::warn!("Cannot write wide string: non-writable at 0x{:x}", to);
            return;
        }
        mem.write_wide_string(to, from);
    }

    #[inline(always)]
    pub fn write_buffer(&mut self, to: u64, from: &[u8]) {
        self.write_bytes_slice(to, from);
    }

    pub fn read_bytes_buff(&self, buff: &mut [u8], addr: u64) {
        let len = buff.len();
        buff.copy_from_slice(self.read_bytes(addr, len));
    }

    #[inline(always)]
    pub fn read_buffer(&mut self, from: u64, sz: usize) -> Vec<u8> {
        self.read_bytes(from, sz).to_vec()
    }

    /// Returns `None` if the range is not fully mapped or not readable.
    pub fn try_read_bytes(&self, addr: u64, sz: usize) -> Option<&[u8]> {
        if sz == 0 {
            return Some(&[]);
        }
        let end_addr = addr + sz as u64 - 1;
        let mem = self.get_mem_by_addr(addr)?;
        if !mem.inside(end_addr) || !mem.can_read() {
            log::warn!(
                "Reading {} bytes from unmapped or non-readable region at 0x{:x}",
                sz,
                addr
            );
            return None;
        }
        Some(mem.read_bytes(addr, sz))
    }

    /// Borrows `sz` bytes from mapped readable memory. Length is always `sz` (for `sz > 0`), so
    /// callers can use `.try_into()` into fixed arrays like before.
    ///
    /// If the range is not readable, logs a warning and returns `sz` zeroed bytes via a small
    /// [`Box::leak`] (avoids panicking; rare bad reads may accumulate leaked memory).
    pub fn read_bytes(&self, addr: u64, sz: usize) -> &[u8] {
        match self.try_read_bytes(addr, sz) {
            Some(s) => s,
            None => {
                if sz == 0 {
                    return &[];
                }
                Box::leak(vec![0u8; sz].into_boxed_slice())
            }
        }
    }

    #[inline(always)]
    pub fn read_bytes_option(&self, addr: u64, sz: usize) -> Option<&[u8]> {
        self.try_read_bytes(addr, sz)
    }

    /// Like [`read_bytes`], but copies into a fixed-size array; uses zeros if the read fails.
    pub fn read_bytes_array<const N: usize>(&self, addr: u64) -> [u8; N] {
        match self.try_read_bytes(addr, N) {
            Some(slice) if slice.len() == N => {
                let mut out = [0u8; N];
                out.copy_from_slice(slice);
                out
            }
            _ => [0u8; N],
        }
    }

    pub fn read_string_of_bytes(&mut self, addr: u64, sz: usize) -> String {
        let mut svec: Vec<String> = Vec::new();
        let bytes = match self.try_read_bytes(addr, sz) {
            Some(b) => b,
            None => return String::new(),
        };
        for bs in bytes.iter() {
            svec.push(format!("{:02x} ", bs));
        }
        let s: String = svec.into_iter().collect();
        s
    }

    pub fn read_string(&self, addr: u64) -> String {
        if addr == 0 {
            return String::new();
        }
        // One map lookup, then scan that map's backing slice for the NUL — vs.
        // the old per-char `read_byte` (a full `get_mem_by_addr` TLB lookup
        // each). This is a top CPU hot path (export/registry name reads).
        match self.get_mem_by_addr(addr) {
            Some(mem) if mem.can_read() => mem.read_string(addr),
            _ => String::new(),
        }
    }

    pub fn read_wide_string_nocrash(&self, addr: u64) -> String {
        if addr == 0 {
            return "".to_string();
        }
        let mem = match self.get_mem_by_addr(addr) {
            Some(m) => m,
            None => {
                return "".to_string();
            }
        };
        mem.read_wide_string(addr)
    }

    pub fn read_wide_string(&self, addr: u64) -> String {
        if addr == 0 {
            return "".to_string();
        }
        let Some(mem) = self.get_mem_by_addr(addr) else {
            log::warn!("read_wide_string: no map at 0x{:x}", addr);
            return String::new();
        };
        if !mem.can_read() {
            log::warn!("read_wide_string: non-readable map at 0x{:x}", addr);
            return String::new();
        }
        mem.read_wide_string(addr)
    }

    pub fn read_wide_string_n(&self, addr: u64, max_chars: usize) -> String {
        if addr == 0 {
            return "".to_string();
        }

        let Some(mem) = self.get_mem_by_addr(addr) else {
            log::warn!("read_wide_string_n: no map at 0x{:x}", addr);
            return String::new();
        };
        if !mem.can_read() {
            log::warn!("read_wide_string_n: non-readable map at 0x{:x}", addr);
            return String::new();
        }
        mem.read_wide_string_n(addr, max_chars)
    }

    pub fn filter_string(&self, s: &mut String) {
        let valid = " 0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \t\x00".as_bytes();
        s.retain(|c| valid.contains(&(c as u8)));
    }

    pub fn filter_replace_bytes(&self, s: &[u8]) -> Vec<u8> {
        let mut sanitized: Vec<u8> = Vec::new();
        let valid =
            "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".as_bytes();
        let mut p;

        for si in s.iter() {
            p = false;
            for validj in valid.iter() {
                if validj == si {
                    sanitized.push(*si);
                    p = true;
                    break;
                }
            }
            if !p {
                sanitized.push(b'.');
            }
        }

        sanitized
    }

    pub fn filter_replace_string(&self, s: &str) -> String {
        let valid =
            "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".as_bytes();
        let sb = s.as_bytes();
        let mut p;
        let mut dst: Vec<char> = Vec::new();

        for i in 0..s.len() {
            p = false;
            for j in 0..valid.len() {
                if sb[i] == valid[j] {
                    dst.push(sb[i] as char);
                    p = true;
                    break;
                }
            }
            if !p {
                dst.push('.');
            }
        }

        let sdst: String = dst.into_iter().collect();
        sdst
    }
}
