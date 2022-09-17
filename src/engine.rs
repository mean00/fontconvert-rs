

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]



extern crate freetype as ft;
extern crate heatshrink;
use ft::FtResult as FtResult;
use heatshrink as hs;
use std::fs::File;
use std::io::{Write};

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
    bpp  : usize,
    bp : bit_pusher::BitPusher,     
    face_height : i8,
    processed_glyphs : Vec <PFXGlyph>,
    compression : bool, 
    uncompressed : usize,
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
                processed_glyphs : Vec::new(),
                bpp  : 0,
                compression : false,
                uncompressed : 0,
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
        self.first = first as usize;
        self.last = last as usize;
        self.compression = compression;
        let status =  self.convertNbit( bpp as usize,map );        
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
    fn convertNbit(&mut self, bpp: usize, map : &[u8;256] ) ->  Result< () , EngineError> 
    {
        self.bpp = bpp;
        let zeroGlyph :  PFXGlyph = PFXGlyph {            bitmapOffset : 0,
                                                            width : 0,
                                                            height : 0,
                                                            xAdvance : 0,
                                                            xOffset : 0,
                                                            yOffset : 0,        };
                
        
        self.processed_glyphs.clear();
        for i in self.first..=self.last
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
                if bpp == 1
                {
                    ok = self.checkOk( self.face.load_char(i , ft::face::LoadFlag::TARGET_MONO ));
                }else
                {
                    ok = self.checkOk( self.face.load_char(i , ft::face::LoadFlag::TARGET_NORMAL ));
                }
            }
            let metrics =  self.face.size_metrics().unwrap();
            self.face_height = (metrics.height / 64) as i8;
            //self.face_height = self.face.size_metrics().unwrap().height as i8;
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
                self.processed_glyphs.push(zeroGlyph.clone());
                continue;
            }
            let gl = glyph.unwrap();
            let mut x_advance=gl.advance_x();
            if x_advance > 0x10000 // workaround a bug in freetype RS
            {
                x_advance = x_advance >> 16;
            }
            let rbitmap: ft::BitmapGlyph;
            if bpp == 1
            {
                rbitmap =  gl.to_bitmap(ft::RenderMode::Mono, None).unwrap();
            }
            else
            {
                rbitmap =  gl.to_bitmap(ft::RenderMode::Normal, None).unwrap();
            }
            let left = rbitmap.left();
            let top = rbitmap.top();
            let bitmap: ft::Bitmap = rbitmap.bitmap();

            let ww=bitmap.width();
            let hh = bitmap.rows();
            let pitch = bitmap.pitch();
            let bits = bitmap.buffer();

            let start_offset = self.bp.size();

            if ww==0 || hh==0
            {
                self.processed_glyphs.push(zeroGlyph.clone());
                continue;
            }      
            let mut thisPFX : PFXGlyph = PFXGlyph::new();
            self.bp.align();
            thisPFX.bitmapOffset = self.bp.size() as u16;
            thisPFX.width = ww as u8;
            thisPFX.height = hh as u8;
            thisPFX.xAdvance = x_advance as u8;
            thisPFX.xOffset = left as i8;
            thisPFX.yOffset = (1 - top) as i8;
            self.processed_glyphs.push(thisPFX);
    
            for y in 0..hh
            {
                let index = y*pitch ;
                let mut pix : u8;
                for x  in 0..ww
                {
                    match bpp
                    {
                        1 => {
                            let v= bits[(index+(x>>3)) as usize];
                            let mask = 0x80>> (x&7);
                            if (v & mask)!=0
                            {
                                self.bp.add1bits(1);
                            }else {
                                self.bp.add1bits(0);
                            }
                        },
                        4 => self.bp.add4bits( bits[ (index+x) as usize] >> 4 ),
                        2 => {  pix= (bits[ (index+x) as usize]+31) >> 6;
                                if pix>3  {pix=3;}
                                self.bp.add2bits(pix as u8);
                             },

                        _ => panic!("oopsie"),
                    }                    
                }                
            }            
        
            self.bp.align();
            println!("Processed {} glyphs, bitmap size {}",self.processed_glyphs.len(),self.bp.size());
            if self.compression
            {
                // in pace packing...
                let original_size = self.bp.size()-start_offset;
                self.uncompressed += original_size;
                let size = self.compressInPlace(start_offset,original_size);
                self.bp.set_offset(start_offset+size);
            }
        }
        Ok(())
    }
    fn compressInPlace(&mut self, offset : usize, size : usize) -> usize
    {
        let cfg = hs::Config::new(8,4).unwrap();
        let mut output : [u8;20*1024]=[0;20*1024];
        let compressed_size = match hs::encode(self.bp.extract(offset,size), 
            &mut output, 
            &cfg)
        {
            Err(x) => 0,
            Ok(x) => {self.bp.truncate(offset);self.bp.swallow(x);x.len()},
        };
        compressed_size
    }
    pub fn dump_bitmap(&mut self, ofile : &mut File, name : &str) -> Result< () ,std::io::Error>
    {
        write!(ofile,"const uint8_t {}Bitmaps[] PROGMEM = {{\n ",name)?;
        self.bp.align();
        let sz = self.bp.size();
      
        let mut tab=0;
        for i in 0..sz
        {
            //print!(" 0x%02X,",data[i]);
            write!(ofile," {:#04X},",self.bp.data(i))?;
            tab=tab+1;
            if tab==12
            {
                write!(ofile,"\n ")?;
                tab=0;
            }
        }
        write!(ofile," }};\n\n")?;
        Ok(())
    }
    pub fn dump_index(&mut self,  ofile : &mut File, name : &str) -> Result< () ,std::io::Error>
    {
      write!(ofile,"const PFXglyph {}Glyphs[] PROGMEM = {{\n", name)?;
      for i in self.first..=self.last
      {
        let glyph=(self.processed_glyphs[i-self.first]).clone();
        write!(ofile,"  {{ {:5}, {:3}, {:3}, {:3}, {:4}, {:4}}}",
               glyph.bitmapOffset,
               glyph.width,
               glyph.height,
               glyph.xAdvance,
               glyph.xOffset,
               glyph.yOffset as isize)?;
               write!(ofile,",   // {:#04x} '{}' \n", i,i as u8 as char)?;
      }
      write!(ofile,"\n}};\n")?;
      Ok(())
    }
    
    


    pub fn dump_footer(&mut self,  ofile : &mut File, name : &str) -> Result< () ,std::io::Error>
    {
  // Output font structure
        write!(ofile,"const PFXfont {} PROGMEM = {{\n", name)?;
        write!(ofile,"  (uint8_t  *){}Bitmaps,\n", name)?;
        write!(ofile,"  (PFXglyph *){}Glyphs,\n", name)?;
        if self.face_height == 0
        {  // No face height info, assume fixed width and get from a glyph.
            write!(ofile,"  {:#04x}, {:#04x}, {}, // first, last, advance (approx)\n" , self.first, self.last, self.processed_glyphs[0].height)?;
        }
        else
        {
            write!(ofile,"  {:#04x}, {:#04x}, {},// first, last, advance\n" , self.first, self.last, self.face_height)?;
        }
        write!(ofile,"  {},{}}}; // bit per pixel, compression \n\n",self.bpp,self.compression as usize)?;
        let sz=self.bp.size();
        if self.compression
        {
            write!(ofile,"// Bitmap uncompressed  : about {} bytes ({} kBytes)\n",self.uncompressed, (self.uncompressed+1023)/1024)?;
            write!(ofile,"// Bitmap output size   : about {} bytes ({} kBytes)\n",sz,(sz+1023)/1024)?;
            write!(ofile,"// Bitmap compression   : {}%\n", (((sz as f32)*100.)/(self.uncompressed as f32)) as u32)?;
        }
        else {
            write!(ofile,"// Bitmap output size   : about {} bytes ({} kBytes)\n",sz,(sz+1023)/1024)?;
        }

        let sizeofglyph =  8; // FIXME BADLY
        let sizeofpfxgly: usize = 16;
        let mut sz=(self.last-self.first+1)*sizeofglyph;
        write!(ofile,"// Header : about {} bytes ({} kBytes)\n",sz,(sz+1023)/1024)?;
        sz=sz+self.bp.size()+sizeofpfxgly;
        write!(ofile,"//--------------------------------------\n")?;
        write!(ofile,"// Total : about {} bytes ({} kBytes)\n",sz,(sz+1023)/1024)?;
        Ok(())
    }
}
// EOF
