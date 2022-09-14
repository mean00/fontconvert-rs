
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

const BUFFER_SIZE: usize = 256*1024;
pub struct BitPusher
{
    bit         : isize,
    accumulator : u8,
    index       : usize,
    buffer      : [u8;BUFFER_SIZE],
}

impl BitPusher
{
    pub fn new() -> BitPusher
    {
        BitPusher{
            bit : 7,
            accumulator : 0,
            index : 0,
            buffer: [0 ; BUFFER_SIZE],
        }
    }
    pub fn swallow(&mut self, data : &[u8]) -> ()
    {
        let n = data.len();
        for i in 0..n
        {
            self.buffer[self.index]=data[i];
            self.index+=1;
        }
    }
    pub fn add8bits(&mut self, val: u8) -> ()
    {
        self.buffer[self.index]=val;
    }
    pub fn add4bits(&mut self, val : u8) -> ()
    {
        if self.bit==7
        {
                self.accumulator |= val;
                self.bit = self.bit -4;
        }else {
            self.accumulator |=(val &0xf) as u8;
            self.align();
        }
    }
    pub fn add2bits(&mut self, val : u8) -> ()
    {
        let rval : u8 = val & 3;
        self.accumulator |= rval << (self.bit -1);
        self.bit = self.bit -2;
        self.checkAlign();
    }
    pub fn add1bits(&mut self, val : u8) -> ()
    {
        if val != 0
        {
            self.accumulator |= 1<<self.bit;
        }
        self.bit =self.bit - 1;
        self.checkAlign();
    }
    fn checkAlign(&mut self) -> ()
    {
        if self.bit < 0
        {
            self.align();
        }
    }
    pub fn align(&mut self) -> ()
    {
        if self.bit ==7 
        {
            return;
        }
        self.buffer[self.index]=self.accumulator;
        self.index = self.index + 1;
        self.bit = 7;
    }
}