use super::Maps;
use std::str;

impl Maps {
    pub fn dump(&self, addr: u64) {
        let mut count = 0;
        for i in 0..8 {
            let mut bytes: Vec<u8> = Vec::new();
            print!("0x{:x}: ", addr + i * 16);
            for _ in 0..16 {
                let b = self.read_byte(addr + count).unwrap_or(0);
                bytes.push(b);
                count += 1;
                print!("{:02x} ", b);
            }

            let pritable_bytes = self.filter_replace_bytes(&bytes);
            let s: String = match str::from_utf8(&pritable_bytes) {
                Ok(v) => v.to_string(),
                Err(_) => " -utf8err- ".to_string(),
            };

            println!("    {}", s);
        }
    }

    pub fn dump_n(&self, addr: u64, amount: u64) {
        let mut count: u64 = 0;
        for i in 0..8 {
            let mut bytes: Vec<u8> = Vec::new();
            print!("0x{:x}: ", addr + i * 16);
            for _ in 0..16 {
                let b = self.read_byte(addr + count).unwrap_or(0);
                bytes.push(b);
                count += 1;
                print!("{:02x} ", b);
                if count >= amount {
                    println!();
                    return;
                }
            }

            let pritable_bytes = self.filter_replace_bytes(&bytes);
            let s: String = match str::from_utf8(&pritable_bytes) {
                Ok(v) => v.to_string(),
                Err(_) => " -utf8err- ".to_string(),
            };

            println!("    {}", s);
        }
    }

    #[deprecated]
    pub fn dump2(&self, addr: u64) {
        let mut count = 0;
        for _ in 0..8 {
            let mut bytes: Vec<u8> = Vec::new();
            print!("0x{:x}: ", addr + count * 4);
            for _ in 0..4 {
                let dw = match self.read_dword(addr + count * 4) {
                    Some(v) => v,
                    None => {
                        log::trace!("bad address");
                        return;
                    }
                };
                count += 1;
                bytes.push((dw & 0xff) as u8);
                bytes.push(((dw & 0xff00) >> 8) as u8);
                bytes.push(((dw & 0xff0000) >> 16) as u8);
                bytes.push(((dw & 0xff000000) >> 24) as u8);
                print!(
                    "{:02x} {:02x} {:02x} {:02x}  ",
                    dw & 0xff,
                    (dw & 0xff00) >> 8,
                    (dw & 0xff0000) >> 16,
                    (dw & 0xff000000) >> 24
                );
            }

            let pritable_bytes = self.filter_replace_bytes(&bytes);
            let s: String = match str::from_utf8(&pritable_bytes) {
                Ok(v) => v.to_string(),
                Err(_) => " -utf8err- ".to_string(),
            };

            log::trace!("{}", s);
        }
    }

    pub fn dump_qwords(&self, addr: u64, n: u64) {
        let mut value: u64;

        for i in 0..n {
            let a = addr + i * 8;
            value = match self.read_qword(a) {
                Some(v) => v,
                None => break,
            };

            let name = self.get_addr_name(value).unwrap_or("");

            log::trace!(
                "0x{:x}: 0x{:x} ({}) '{}'",
                a,
                value,
                name,
                self.filter_replace_string(&self.read_string(value))
            );
        }
    }

    pub fn dump_dwords(&self, addr: u64, n: u64) {
        let mut value: u32;

        for i in 0..n {
            let a = addr + i * 4;
            value = match self.read_dword(a) {
                Some(v) => v,
                None => break,
            };

            if !self.is_64bits {
                // only in 32bits make sense derreference dwords in memory
                let name = self.get_addr_name(value.into()).unwrap_or("");

                let mut s = "".to_string();
                if !name.is_empty() {
                    s = self.read_string(value.into());
                }

                log::trace!(
                    "0x{:x}: 0x{:x} ({}) '{}'",
                    a,
                    value,
                    name,
                    self.filter_replace_string(&s)
                );
            } else {
                log::trace!("0x{:x}: 0x{:x}", a, value);
            }
        }
    }
}
