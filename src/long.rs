use std::collections::VecDeque;
use std::io;

pub type Long = i64;

pub trait ReadLong<E> {
    fn read_long(&mut self) -> Result<Long, E>;
}

pub trait WriteLong {
    fn write_long(&self) -> VecDeque<u8>;
}

impl ReadLong<io::Error> for VecDeque<u8> {
    fn read_long(&mut self) -> Result<Long, io::Error> {
        let mut result: Long = 0;

        for _ in 1..=8 {
            result += self
                .pop_front()
                .expect("Vector needs to have 8 bytes to decode a long(i64).")
                as i64;
            result = result << 8;
        }

        Ok(result)
    }
}

impl WriteLong for i64 {
    fn write_long(&self) -> VecDeque<u8> {
        let mut result = VecDeque::with_capacity(8);

        let mut value = *self;

        for _ in 1..=8 {
            let temp = value & 0b11111111;

            value = value >> 8;

            result.push_front(temp as u8);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::{Long, ReadLong, WriteLong};
    use std::collections::VecDeque;

    #[test]
    fn test_read_long_on_vec() {
        let mappings: Vec<(Long, Vec<u8>)> = vec![
            (632469504, vec![0, 0, 0, 0, 0, 37, 178, 184]),
            (631943936, vec![0, 0, 0, 0, 0, 37, 170, 179]),
            (630137600, vec![0, 0, 0, 0, 0, 37, 143, 35]),
        ];

        for mapping in mappings {
            assert_eq!(mapping.0, VecDeque::from(mapping.1).read_long().unwrap());
        }
    }
    #[test]
    fn test_write_long_to_vec() {
        let mappings: Vec<(Long, Vec<u8>)> = vec![
            (632469504, vec![0, 0, 0, 0, 0, 37, 178, 184]),
            (631943936, vec![0, 0, 0, 0, 0, 37, 170, 179]),
            (630137600, vec![0, 0, 0, 0, 0, 37, 143, 35]),
        ];

        for mapping in mappings {
            assert_eq!(VecDeque::from(mapping.1), mapping.0.write_long());
        }
    }
}