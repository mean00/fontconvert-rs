

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]



extern crate freetype as ft;
use ft::FtResult as FtResult;
use crate::bit_pusher;
const DPI: u32 = 141; // Approximate res. of Adafruit 2.8" TFT


#[derive(Debug)]

pub enum EngineError {
    FreeTypeError,
    FileError,
    InternalError,
  }
#[derive(Debug, Clone)]
struct  PFXGlyph {
    pub bitmapOffset : u16,
    pub width : u8,
    pub height : u8,
    pub xAdvance : u8,
    pub xOffset : i8,
    pub yOffset : i8,
}
impl PFXGlyph
{
    fn new () -> PFXGlyph
    {
        PFXGlyph{
            bitmapOffset : 0,
            width : 0,
            height : 0,
            xAdvance : 0,
            yOffset : 0,
            xOffset : 0,
        }
    }
}
pub struct Engine
{
    lib  :  ft::Library,
    face :  ft::Face, //&'static ft::Face <'static> ,   
    first:  usize,     
    last:  usize,
    bp : bit_pusher::BitPusher,     
    face_height : i8,
}

/// Engine is the engine to convert TTF font
/// to bitmap font
/// - font : Complete path to the ttf string to use
/// - size : Size of the font in pixels
/// 
impl  Engine  
{
    ///
    /// Constuctor
    pub fn new<'b>( font : &String, size: usize  ) ->   Result<Engine , EngineError> 
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
                first : 32,
                last  : 128,
                bp : bit_pusher::BitPusher::new(),
                face_height : 0,
        };
        Ok(e)
    }
    /// run
    /// bpp : Bit per pixel : 1, 2 or 4
    /// compression : Heatshrink based compression
    /// map  : Map of glyphs to render [x]=0 => dont render, [x]=1 => render
    /// 
    pub fn run( &mut self,bpp: u8, compression: bool , map : &[u8;256]  ) ->   Result< () , EngineError> 
    {
        // scan map to find the 1st and last to process
        let mut first : isize = -1;
        let mut last : isize = 256;
        for i in 0..256
        {   
            if map[i]==0
            {         
              continue;
            }
            if first==-1
            {
                    first=i as isize;
            }
            last=i as isize;
        }
      
        let status = match bpp
        {
            1 => self.convert1bit(first as usize, last as usize, map  ),
            _ => Err(EngineError::InternalError)?,
        };
        Ok(())
    }
    ///
    /// 
    /// 
    /// 

    fn checkOk( &self, t: FtResult<()> ) -> bool
    {
        match t
        {
            Err(r) => false,
            _      => true,
        }
    }
    ///
    /// 
    /// 
    fn convert1bit(&mut self, first : usize, last  : usize, map : &[u8;256] ) ->  Result< () , EngineError> 
    {
        let zeroGlyph :  PFXGlyph = PFXGlyph {            bitmapOffset : 0,
                                                            width : 0,
                                                            height : 0,
                                                            xAdvance : 0,
                                                            xOffset : 0,
                                                            yOffset : 0,        };
                
        let mut processed_glyphs : Vec <PFXGlyph> = Vec::new();
        for i in first..last
        {
            
            let mut ok : bool =true;
            let mut skipped: bool =false;
            let mut glyph : Option<ft::Glyph>=None;
            if map[i]==0
            {                
                ok=false;
                skipped=true;
            }
           
            if ok
            {
                ok = self.checkOk( self.face.load_char(i , ft::face::LoadFlag::TARGET_MONO ));
            }
            self.face_height = self.face.size_metrics().unwrap().height as i8;
           // if ok
           // {
           //     ok = self.checkOk(  self.face.glyph().render_glyph( ft::RenderMode::Normal));
           // }            
            if ok
            {                
                let r= self.face.glyph().get_glyph();
                match r
                {
                    Err(r) => ok=false ,
                    Ok(glyph2) => glyph=Some(glyph2) ,
                };                        
            }     
            // Ok glyph contains the rendered glyph      
            if ok == false && skipped == false 
            {
                // WARNING / ERROR
                println!("Failed to render glyph {}",i);                
            }
            if ok==false
            {
                processed_glyphs.push(zeroGlyph.clone());
                continue;
            }
            let gl = glyph.unwrap();
            let x_advance=gl.advance_x();
            let rbitmap: ft::BitmapGlyph=  gl.to_bitmap(ft::RenderMode::Mono, None).unwrap();                        
            let left = rbitmap.left();
            let top = rbitmap.top();
            let bitmap: ft::Bitmap = rbitmap.bitmap();

            let ww=bitmap.width();
            let hh = bitmap.rows();
            let pitch = bitmap.pitch();
            let bits = bitmap.buffer();

            if ww==0 || hh==0
            {
                processed_glyphs.push(zeroGlyph.clone());
                continue;
            }      
            let mut thisPFX : PFXGlyph = PFXGlyph::new();
            self.bp.align();
            thisPFX.bitmapOffset = self.bp.size() as u16;
            thisPFX.width = ww as u8;
            thisPFX.height = hh as u8;
            thisPFX.xAdvance = (x_advance >> 6) as u8;
            thisPFX.xOffset = left as i8;
            thisPFX.yOffset = (1 - top) as i8;
            processed_glyphs.push(thisPFX);
    
            for y in 0..hh
            {
                let index = y*pitch ;
                for x in 0..ww
                {
                        let v= bits[(index+(x>>3)) as usize];
                        let mask = 0x80>> (x&7);
                        if (v & mask)!=0
                        {
                            self.bp.add1bits(1);
                        }else {
                            self.bp.add1bits(0);
                        }
                }                
            }            
        }
        self.bp.align();
        println!("Processed {} glyphs, bitmap size {}",processed_glyphs.len(),self.bp.size());
        Ok(())
    }
}
// EOF
