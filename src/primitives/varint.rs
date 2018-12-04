use bytes::{Bytes, Buf, IntoBuf};
use std::ops::Add;
use std::ops::Deref;

pub struct VarInt(u64);

impl VarInt {

    pub fn new(n: u64) -> VarInt {
        VarInt(n)
    }

    pub fn len(&self) -> usize {
        let mut n = self.0;
        let mut n_ret: usize = 0;
        loop {
            n_ret += 1;
            if n <= 0x7F { break; }
            n = (n >> 7) - 1;
            }
        return n_ret;
    }

    // TODO: Change to result
    pub fn parse(raw: &[u8]) -> VarInt {
        let mut n: u64 = 0;
        let mut buf = raw.into_buf();
        loop {
            let k = buf.get_u8();
            n = (n << 7) | ((k & 0x7f) as u64);
            if 0x00 != (k & 0x80) {
                n += 1;
            } else {
                return VarInt::new(n);
            }
        }
    }
}

impl Add for VarInt {
    type Output = VarInt;

    fn add(self, other: VarInt) -> VarInt {
        VarInt(self.0 + u64::from(other))
    }
}

impl From<usize> for VarInt {fn from(item: usize) -> Self {VarInt(item as u64)}}

impl From<VarInt> for usize {fn from(item: VarInt) -> Self {item.0 as usize}}

impl From<u64> for VarInt {fn from(item: u64) -> Self {VarInt(item)}}

impl From<VarInt> for u64 {fn from(item: VarInt) -> Self {item.0 as u64}}

impl Clone for VarInt {fn clone(&self) -> VarInt { VarInt(self.0) }}