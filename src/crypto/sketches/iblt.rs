// TODO: Eventually replace this with minisketch
// TODO: Optimize this

use bytes::Bytes;
use crypto::hashes::blake2b::*;
use crypto::util::*;
use std::collections::HashSet;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use utils::byte_ops::*;
use utils::constants::*;
use crypto::sketches::odd_sketch::*;
use utils::serialisation::*;

#[derive(PartialEq, Clone, Debug)]
struct Row {
    count: i16,
    payload: Bytes,
    checksum: Bytes,
}

impl Row {
    pub fn empty_row() -> Row {
        Row {
            count: 0,
            payload: Bytes::from(&[0; IBLT_PAYLOAD_LEN][..]),
            checksum: Bytes::from(&[0; IBLT_CHECKSUM_LEN][..]),
        }
    }

    pub fn unit_row(payload: &Bytes) -> Row {
        Row {
            count: 1,
            payload: payload.clone(),
            checksum: payload.blake2b().slice_to(IBLT_CHECKSUM_LEN),
        }
    }

    pub fn count_row(payload: &Bytes, count: i16) -> Row {
        Row {
            count,
            payload: payload.clone(),
            checksum: payload.blake2b().slice_to(IBLT_CHECKSUM_LEN),
        }
    }

    pub fn is_pure(&self) -> bool {
        (self.count == 1 || self.count == -1)
            && (self.checksum == self.payload.blake2b().slice_to(IBLT_CHECKSUM_LEN))
    }

    pub fn is_empty(&self) -> bool {
        (self.count == 0)
            && (self.payload.iter().all(|&x| x == 0))
            && (self.checksum.iter().all(|&x| x == 0))
    }
}

impl Add for Row {
    type Output = Row;

    fn add(self, other: Row) -> Row {
        Row {
            count: self.count + other.count,
            payload: self.payload.byte_xor(other.payload),
            checksum: self.checksum.byte_xor(other.checksum),
        }
    }
}

impl AddAssign for Row {
    fn add_assign(&mut self, other: Row) {
        *self = Row {
            count: self.count + other.count,
            payload: self.payload.clone().byte_xor(other.payload),
            checksum: self.checksum.clone().byte_xor(other.checksum),
        };
    }
}

impl Sub for Row {
    type Output = Row;

    fn sub(self, other: Row) -> Row {
        Row {
            count: self.count - other.count,
            payload: self.payload.byte_xor(other.payload),
            checksum: self.checksum.byte_xor(other.checksum),
        }
    }
}

impl SubAssign for Row {
    fn sub_assign(&mut self, other: Row) {
        *self = Row {
            count: self.count - other.count,
            payload: self.payload.clone().byte_xor(other.payload),
            checksum: self.checksum.clone().byte_xor(other.checksum),
        };
    }
}

#[derive(Clone, Debug)]
pub struct IBLT {
    n_hashes: usize,
    rows: Vec<Row>,
}

impl Sub for IBLT {
    type Output = IBLT;

    fn sub(self, other: IBLT) -> IBLT {
        IBLT {
            n_hashes: self.n_hashes,
            rows: self
                .rows
                .into_iter()
                .zip(other.rows.into_iter())
                .map(|(row_a, row_b)| row_a - row_b)
                .collect(),
        }
    }
}

impl IBLT {
    pub fn with_capacity(capacity: usize, n_hashes: usize) -> IBLT {
        IBLT {
            n_hashes,
            rows: vec![Row::empty_row(); capacity],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.rows.iter().all(|row| row.is_empty())
    }

    pub fn get_pure(&self) -> Option<Row> {
        self.rows.iter().find(|row| row.is_pure()).cloned()
    }

    pub fn insert(&mut self, payload: Bytes) {
        let len = self.rows.len();
        for i in (0..self.n_hashes).map(|k| get_pos(&payload, k, len)) {
            self.rows[i] += Row::unit_row(&payload);
        }
    }

    pub fn decode(&self) -> Result<(HashSet<Bytes>, HashSet<Bytes>), String> {
        let mut left = HashSet::with_capacity(self.rows.len());
        let mut right = HashSet::with_capacity(self.rows.len());

        let mut decode_iblt = self.clone();

        loop {
            if let Some(row) = decode_iblt.get_pure() {
                let payload = row.clone().payload;
                let count = row.clone().count;

                if count > 0 {
                    left.insert(payload.clone());
                } else {
                    right.insert(payload.clone());
                }

                for j in (0..self.n_hashes).map(|k| get_pos(&payload, k, self.rows.len())) {
                    decode_iblt.rows[j] -= Row::count_row(&payload, count);
                }
            } else {
                break;
            }
        }

        if decode_iblt.is_empty() {
            Ok((left, right))
        } else {
            Err("Failed to decode".to_string())
        }
    }
}

impl PartialEq<Bytes> for IBLT {
    fn eq(&self, other: &Bytes) -> bool {
        let (hash_set_l, _) = match self.decode() {
            Ok(hs) => hs,
            Err(_) => return false,
        };
        *other == hash_set_l.odd_sketch()
    }
}