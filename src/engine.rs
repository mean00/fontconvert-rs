

#![allow(dead_code)]
#![allow(unused_variables)]


extern crate freetype as ft;

const DPI: u32 = 141; // Approximate res. of Adafruit 2.8" TFT


#[derive(Debug)]

pub enum EngineError {
    FreeTypeError,
    FileError,
  }


pub struct Engine 
{
    lib  :  ft::Library,
    
    //face : ft::library: 
}


impl Engine 
{
    pub fn new( font : &String, size: usize) -> Result<Engine, EngineError>
    {
        let r = ft::Library::init();
        let lib = match r 
        {
            Err(x) => Err(EngineError::FreeTypeError)?,
            Ok(x) => x,
        };


        let face = lib.new_face(font, 0);
        let face = match face
        {
            Err(x) => Err(EngineError::FreeTypeError)?,
            Ok(x) => x,
        };

        let fsize=face.set_char_size((size<<6) as isize,  0 as isize,  DPI as u32, 0 as u32);
        match fsize
        {
            Err(x) => Err(EngineError::FreeTypeError)?,
            Ok(x) => x,
        }

        let e : Engine=Engine
        {
                lib:  lib,
                
        };
        Ok(e)
    }
}