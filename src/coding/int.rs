use super::Encodeable;
use std::collections::VecDeque;

pub type MinecraftInt = i32;

impl Encodeable for MinecraftInt {
    fn encode(&self) -> VecDeque<u8> {
        // an int is 4 bytes long
        let mut result: VecDeque<u8> = VecDeque::with_capacity(4);

        let mut value = *self;

        for _ in 1..=4 {
            let temp = value & 0b1111;

            value >>= 4;

            result.push_back(temp as u8);
        }

        result
    }

    fn byte_length(&self) -> u8 {
        4
    }
}
