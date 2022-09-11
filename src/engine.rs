

#![allow(dead_code)]
#![allow(unused_variables)]


extern crate freetype as ft;

const DPI: u32 = 141; // Approximate res. of Adafruit 2.8" TFT


#[derive(Debug)]

pub enum EngineError {
    FreeTypeError,
    FileError,
  }


pub struct Engine<'a>
{
    lib  :  ft::Library,
    face :  ft::Face<'a>, //&'static ft::Face <'static> ,    
    bpp  :  usize,
    mapp :  [u8;256],
}

/// Engine is the engine to convert TTF font
/// to bitmap font
/// 
impl <'a> Engine  <'a>
{
    ///
    /// Constuctor
    pub fn new<'b>( font : &String, size: usize,  bbpp: u8, bmapp : &'b[ u8 ]  ) ->   Result<Engine <'a>, EngineError> 
    {
        let r = ft::Library::init();
        let lib = match r 
        {
            Err(x) => Err(EngineError::FreeTypeError)?,
            Ok(x) => x,
        };

//  FT_UInt interpreter_version = TT_INTERPRETER_VERSION_35;
//  FT_Property_Set(library, "truetype", "interpreter-version", &interpreter_version);


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
                face: face,
                bpp:  bbpp as usize,
                mapp: bmapp.clone(),
                
        };
        Ok(e)
    }
}