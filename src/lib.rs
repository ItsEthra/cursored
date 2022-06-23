use std::io::{Cursor, Read};

pub struct Cursored {
    c: Cursor<Vec<u8>>
}

impl Cursored {
    pub fn new(vec: Vec<u8>) -> Self {
        Self {
            c: Cursor::new(vec.into())
        }
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.c.into_inner()
    }

    pub fn remaining(&self) -> usize {
        self.c.get_ref().len().saturating_sub(self.c.position() as usize)
    }

    pub fn lasting(&self) -> &[u8] {
        &self.c.get_ref()[self.c.position() as usize..]
    }

    pub fn advance(&mut self, next: i64) {
        if next < 0 && self.c.position() < -next as u64 {
            panic!("Tried to advance past start.");
        } else if next > 0 && (self.c.get_ref().len() as u64) < self.c.position().wrapping_add(next as u64) {
            panic!("Tried to advance past end.");
        }

        self.c.set_position(self.c.position().wrapping_add(next as u64));
    }
}

impl Cursored {
    pub fn read_slice(&mut self, len: usize) -> &[u8] {
        self.c.set_position(self.c.position() + len as u64);
        &self.c.get_ref()[self.c.position() as usize - len..self.c.position() as usize]
    }

    pub fn read_u8(&mut self) -> u8 {
        let mut buf = [0];
        self.c.read_exact(&mut buf).expect("Exhausted");
        buf[0]
    }

    pub fn read_i8(&mut self) -> i8 {
        let mut buf = [0];
        self.c.read_exact(&mut buf).expect("Exhausted");
        buf[0] as i8
    }

    pub fn read_u16(&mut self) -> u16 {
        let mut buf = [0; 2];
        self.c.read_exact(&mut buf).expect("Exhausted");
        u16::from_be_bytes(buf)
    }

    pub fn read_u16_le(&mut self) -> u16 {
        let mut buf = [0; 2];
        self.c.read_exact(&mut buf).expect("Exhausted");
        u16::from_le_bytes(buf)
    }

    pub fn read_i16(&mut self) -> i16 {
        let mut buf = [0; 2];
        self.c.read_exact(&mut buf).expect("Exhausted");
        i16::from_be_bytes(buf)
    }

    pub fn read_i16_le(&mut self) -> i16 {
        let mut buf = [0; 2];
        self.c.read_exact(&mut buf).expect("Exhausted");
        i16::from_le_bytes(buf)
    }

    pub fn read_u32(&mut self) -> u32 {
        let mut buf = [0; 4];
        self.c.read_exact(&mut buf).expect("Exhausted");
        u32::from_be_bytes(buf)
    }

    pub fn read_u32_le(&mut self) -> u32 {
        let mut buf = [0; 4];
        self.c.read_exact(&mut buf).expect("Exhausted");
        u32::from_le_bytes(buf)
    }

    pub fn read_i32(&mut self) -> i32 {
        let mut buf = [0; 4];
        self.c.read_exact(&mut buf).expect("Exhausted");
        i32::from_be_bytes(buf)
    }

    pub fn read_i32_le(&mut self) -> i32 {
        let mut buf = [0; 4];
        self.c.read_exact(&mut buf).expect("Exhausted");
        i32::from_le_bytes(buf)
    }

    pub fn read_u64(&mut self) -> u64 {
        let mut buf = [0; 8];
        self.c.read_exact(&mut buf).expect("Exhausted");
        u64::from_be_bytes(buf)
    }

    pub fn read_u64_le(&mut self) -> u64 {
        let mut buf = [0; 8];
        self.c.read_exact(&mut buf).expect("Exhausted");
        u64::from_le_bytes(buf)
    }

    pub fn read_i64(&mut self) -> i64 {
        let mut buf = [0; 8];
        self.c.read_exact(&mut buf).expect("Exhausted");
        i64::from_be_bytes(buf)
    }

    pub fn read_i64_le(&mut self) -> i64 {
        let mut buf = [0; 8];
        self.c.read_exact(&mut buf).expect("Exhausted");
        i64::from_le_bytes(buf)
    }

    pub fn read_u128(&mut self) -> u128 {
        let mut buf = [0; 16];
        self.c.read_exact(&mut buf).expect("Exhausted");
        u128::from_be_bytes(buf)
    }

    pub fn read_u128_le(&mut self) -> u128 {
        let mut buf = [0; 16];
        self.c.read_exact(&mut buf).expect("Exhausted");
        u128::from_le_bytes(buf)
    }

    pub fn read_i128(&mut self) -> i128 {
        let mut buf = [0; 16];
        self.c.read_exact(&mut buf).expect("Exhausted");
        i128::from_be_bytes(buf)
    }

    pub fn read_i128_le(&mut self) -> i128 {
        let mut buf = [0; 16];
        self.c.read_exact(&mut buf).expect("Exhausted");
        i128::from_le_bytes(buf)
    }

    pub fn read_f32(&mut self) -> f32 {
        let mut buf = [0; 4];
        self.c.read_exact(&mut buf).expect("Exhausted");
        f32::from_be_bytes(buf)
    }

    pub fn read_f32_le(&mut self) -> f32 {
        let mut buf = [0; 4];
        self.c.read_exact(&mut buf).expect("Exhausted");
        f32::from_le_bytes(buf)
    }

    pub fn read_f64(&mut self) -> f64 {
        let mut buf = [0; 8];
        self.c.read_exact(&mut buf).expect("Exhausted");
        f64::from_be_bytes(buf)
    }

    pub fn read_f64_le(&mut self) -> f64 {
        let mut buf = [0; 8];
        self.c.read_exact(&mut buf).expect("Exhausted");
        f64::from_le_bytes(buf)
    }
}

impl Cursored {
    pub fn put_slice(&mut self, slice: &[u8]) {
        self.c.get_mut().extend_from_slice(slice);
    }

    pub fn put_u8(&mut self, val: u8) {
        self.c.get_mut().push(val);
    }

    pub fn put_i8(&mut self, val: i8) {
        self.c.get_mut().push(val as u8);
    }

    pub fn put_u16(&mut self, val: u16) {
        self.c.get_mut().extend_from_slice(&val.to_be_bytes());
    }

    pub fn put_u16_le(&mut self, val: u16) {
        self.c.get_mut().extend_from_slice(&val.to_le_bytes());
    }

    pub fn put_i16(&mut self, val: i16) {
        self.c.get_mut().extend_from_slice(&val.to_be_bytes());
    }

    pub fn put_i16_le(&mut self, val: i16) {
        self.c.get_mut().extend_from_slice(&val.to_le_bytes());
    }

    pub fn put_u32(&mut self, val: u32) {
        self.c.get_mut().extend_from_slice(&val.to_be_bytes());
    }

    pub fn put_u32_le(&mut self, val: u32) {
        self.c.get_mut().extend_from_slice(&val.to_le_bytes());
    }

    pub fn put_i32(&mut self, val: i32) {
        self.c.get_mut().extend_from_slice(&val.to_be_bytes());
    }

    pub fn put_i32_le(&mut self, val: i32) {
        self.c.get_mut().extend_from_slice(&val.to_le_bytes());
    }

    pub fn put_u64(&mut self, val: u64) {
        self.c.get_mut().extend_from_slice(&val.to_be_bytes());
    }

    pub fn put_u64_le(&mut self, val: u64) {
        self.c.get_mut().extend_from_slice(&val.to_le_bytes());
    }

    pub fn put_i64(&mut self, val: i64) {
        self.c.get_mut().extend_from_slice(&val.to_be_bytes());
    }

    pub fn put_i64_le(&mut self, val: i64) {
        self.c.get_mut().extend_from_slice(&val.to_le_bytes());
    }

    pub fn put_u128(&mut self, val: u128) {
        self.c.get_mut().extend_from_slice(&val.to_be_bytes());
    }

    pub fn put_u128_le(&mut self, val: u128) {
        self.c.get_mut().extend_from_slice(&val.to_le_bytes());
    }

    pub fn put_i128(&mut self, val: i128) {
        self.c.get_mut().extend_from_slice(&val.to_be_bytes());
    }

    pub fn put_i128_le(&mut self, val: i128) {
        self.c.get_mut().extend_from_slice(&val.to_le_bytes());
    }

    pub fn put_f32(&mut self, val: f32) {
        self.c.get_mut().extend_from_slice(&val.to_be_bytes());
    }

    pub fn put_f32_le(&mut self, val: f32) {
        self.c.get_mut().extend_from_slice(&val.to_le_bytes());
    }

    pub fn put_f64(&mut self, val: f64) {
        self.c.get_mut().extend_from_slice(&val.to_be_bytes());
    }

    pub fn put_f64_le(&mut self, val: f64) {
        self.c.get_mut().extend_from_slice(&val.to_le_bytes());
    }
}