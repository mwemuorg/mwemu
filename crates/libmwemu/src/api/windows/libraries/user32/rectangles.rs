use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::memory::read_array;
use crate::winapi::policy::api_try;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

impl Rect {
    fn read(emu: &Emu, addr: u64) -> Option<Self> {
        if addr == 0 {
            return None;
        }
        let bytes = read_array::<{ 4 * size_of::<i32>() }>(emu, addr)?;
        let (words, _) = bytes.as_chunks::<4>();
        let [left, top, right, bottom] = std::array::from_fn(|i| i32::from_le_bytes(words[i]));
        Some(Self {
            left,
            top,
            right,
            bottom,
        })
    }

    fn write(self, emu: &mut Emu, addr: u64) -> bool {
        let bytes = [self.left, self.top, self.right, self.bottom].map(i32::to_le_bytes);
        let bytes = bytes.as_flattened();
        addr != 0
            && emu.maps.validate_write_range(addr, bytes.len() as u64)
            && emu.maps.write_bytes(addr, bytes)
    }

    fn empty(self) -> bool {
        self.left >= self.right || self.top >= self.bottom
    }

    fn intersection(self, b: Self) -> Self {
        let r = Self {
            left: self.left.max(b.left),
            top: self.top.max(b.top),
            right: self.right.min(b.right),
            bottom: self.bottom.min(b.bottom),
        };
        if r.empty() { Self::default() } else { r }
    }

    fn union(self, b: Self) -> Self {
        match (self.empty(), b.empty()) {
            (true, true) => Self::default(),
            (true, false) => b,
            (false, true) => self,
            (false, false) => Self {
                left: self.left.min(b.left),
                top: self.top.min(b.top),
                right: self.right.max(b.right),
                bottom: self.bottom.max(b.bottom),
            },
        }
    }

    fn subtract(mut self, b: Self) -> Self {
        let cut = self.intersection(b);
        if self.empty() || cut == self {
            return Self::default();
        }
        if cut.empty() {
            return self;
        }
        if cut.top == self.top && cut.bottom == self.bottom {
            if cut.left == self.left {
                self.left = cut.right;
            } else if cut.right == self.right {
                self.right = cut.left;
            }
        } else if cut.left == self.left && cut.right == self.right {
            if cut.top == self.top {
                self.top = cut.bottom;
            } else if cut.bottom == self.bottom {
                self.bottom = cut.top;
            }
        }
        self
    }
}

pub fn SetRect(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let addr = api_try!(emu, call.arg(emu, 0));
    let rect = Rect {
        left: api_try!(emu, call.arg_i32(emu, 1)),
        top: api_try!(emu, call.arg_i32(emu, 2)),
        right: api_try!(emu, call.arg_i32(emu, 3)),
        bottom: api_try!(emu, call.arg_i32(emu, 4)),
    };
    let ok = rect.write(emu, addr);
    log_red!(emu, "user32!SetRect rect={addr:#x}");
    call.set_return(emu, ok.into());
    call.cleanup_args(emu, 5);
}

pub fn SetRectEmpty(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let addr = api_try!(emu, call.arg(emu, 0));
    let ok = Rect::default().write(emu, addr);
    log_red!(emu, "user32!SetRectEmpty rect={addr:#x}");
    call.set_return(emu, ok.into());
    call.cleanup_args(emu, 1);
}

pub fn CopyRect(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [dest, src] = api_try!(emu, call.args(emu));
    let ok = Rect::read(emu, src).is_some_and(|r| r.write(emu, dest));
    log_red!(emu, "user32!CopyRect source={src:#x} destination={dest:#x}");
    call.set_return(emu, ok.into());
    call.cleanup_args(emu, 2);
}

pub fn IsRectEmpty(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let addr = api_try!(emu, call.arg(emu, 0));
    let empty = Rect::read(emu, addr).is_none_or(Rect::empty);
    log_red!(emu, "user32!IsRectEmpty rect={addr:#x} empty={empty}");
    call.set_return(emu, empty.into());
    call.cleanup_args(emu, 1);
}

pub fn EqualRect(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [a, b] = api_try!(emu, call.args(emu));
    let same = Rect::read(emu, a)
        .zip(Rect::read(emu, b))
        .is_some_and(|(a, b)| a == b);
    log_red!(
        emu,
        "user32!EqualRect first={a:#x} second={b:#x} equal={same}"
    );
    call.set_return(emu, same.into());
    call.cleanup_args(emu, 2);
}

pub fn PtInRect(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let addr = api_try!(emu, call.arg(emu, 0));
    let point = api_try!(emu, call.arg_u64(emu, 1));
    let x = point as i32;
    let y = (point >> i32::BITS) as i32;
    let inside = Rect::read(emu, addr)
        .is_some_and(|r| (r.left..r.right).contains(&x) && (r.top..r.bottom).contains(&y));
    log_red!(
        emu,
        "user32!PtInRect rect={addr:#x} x={x} y={y} inside={inside}"
    );
    call.set_return(emu, inside.into());
    call.cleanup_args(emu, 3);
}

fn adjust(emu: &mut Emu, name: &str, inflate: bool) {
    let call = WinApi(Stdcall);
    let addr = api_try!(emu, call.arg(emu, 0));
    let dx = api_try!(emu, call.arg_i32(emu, 1));
    let dy = api_try!(emu, call.arg_i32(emu, 2));
    let ok = Rect::read(emu, addr).is_some_and(|mut r| {
        r.left = r
            .left
            .wrapping_add(if inflate { dx.wrapping_neg() } else { dx });
        r.top = r
            .top
            .wrapping_add(if inflate { dy.wrapping_neg() } else { dy });
        r.right = r.right.wrapping_add(dx);
        r.bottom = r.bottom.wrapping_add(dy);
        r.write(emu, addr)
    });
    log_red!(emu, "user32!{name} rect={addr:#x} dx={dx} dy={dy}");
    call.set_return(emu, ok.into());
    call.cleanup_args(emu, 3);
}

pub fn InflateRect(emu: &mut Emu) {
    adjust(emu, "InflateRect", true);
}
pub fn OffsetRect(emu: &mut Emu) {
    adjust(emu, "OffsetRect", false);
}

fn combine(emu: &mut Emu, name: &str, operation: fn(Rect, Rect) -> Rect) {
    let call = WinApi(Stdcall);
    let [dest, a, b] = api_try!(emu, call.args(emu));
    let ok = Rect::read(emu, a)
        .zip(Rect::read(emu, b))
        .is_some_and(|(a, b)| {
            let r = operation(a, b);
            r.write(emu, dest) && !r.empty()
        });
    log_red!(
        emu,
        "user32!{name} first={a:#x} second={b:#x} destination={dest:#x}"
    );
    call.set_return(emu, ok.into());
    call.cleanup_args(emu, 3);
}

pub fn IntersectRect(emu: &mut Emu) {
    combine(emu, "IntersectRect", Rect::intersection);
}
pub fn UnionRect(emu: &mut Emu) {
    combine(emu, "UnionRect", Rect::union);
}
pub fn SubtractRect(emu: &mut Emu) {
    combine(emu, "SubtractRect", Rect::subtract);
}
