

#![allow(dead_code)]
#![allow(unused_variables)]


extern crate freetype as ft;

const DPI: u32 = 141; // Approximate res. of Adafruit 2.8" TFT


#[derive(Debug)]

pub enum EngineError {
    FreeTypeError,
    FileError,
    InternalError,
  }


pub struct Engine<'a>
{
    lib  :  ft::Library,
    face :  ft::Face<'a>, //&'static ft::Face <'static> ,   
    first:  usize,     
    last:  usize,
}

/// Engine is the engine to convert TTF font
/// to bitmap font
/// - font : Complete path to the ttf string to use
/// - size : Size of the font in pixels
/// 
impl <'a> Engine  <'a>
{
    ///
    /// Constuctor
    pub fn new<'b>( font : &String, size: usize  ) ->   Result<Engine <'a>, EngineError> 
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
                last  : 128
        };
        Ok(e)
    }
    /// run
    /// bpp : Bit per pixel : 1, 2 or 4
    /// compression : Heatshrink based compression
    /// map  : Map of glyphs to render [x]=0 => dont render, [x]=1 => render
    /// 
    pub fn run( &mut self,bpp: usize, compression: bool , map : &[u8;256]  ) ->   Result< () , EngineError> 
    {
        // scan map to find the 1st and last to process
        let first : isize = -1;
        let last : isize = 256;
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
    fn convert1bit(&mut self, first : usize, last  : usize, map : &[u8;256] ) ->  Result< () , EngineError> 
    {
        let zeroGlyph: ft::Glyph  ;
        let glyph:     ft::Glyph  ;

        for i in first..last
        {
            if map[i]==0
            {
                //listOfGlyphs.push_back(zeroGlyph);
                continue;
            }
        }

        Ok(())
    }
}
     /*
         // MONO renderer provides clean image with perfect crop
         // (no wasted pixels) via bitmap struct.
         bool renderingOk=true;
         if ((err = FT_Load_Char(face, i, FT_LOAD_TARGET_MONO))) {     fprintf(stderr, "Error %d loading char '%c'\n", err, i); renderingOk=false;   }
         if ((err = FT_Render_Glyph(face->glyph, FT_RENDER_MODE_MONO))) {      fprintf(stderr, "Error %d rendering char '%c'\n", err, i);     renderingOk=false;  }
         if ((err = FT_Get_Glyph(face->glyph, &glyph))) {      fprintf(stderr, "Error %d getting glyph '%c'\n", err, i);    renderingOk=false;    }
 
         if(!renderingOk)
         {
             listOfGlyphs.push_back(zeroGlyph);
             continue;
         }
         FT_Bitmap *bitmap = &face->glyph->bitmap;
         FT_BitmapGlyphRec *g= (FT_BitmapGlyphRec *)glyph;
 
         // Minimal font and per-glyph information is stored to
         // reduce flash space requirements.  Glyph bitmaps are
         // fully bit-packed; no per-scanline pad, though end of
         // each character may be padded to next byte boundary
         // when needed.  16-bit offset means 64K max for bitmaps,
         // code currently doesn't check for overflow.  (Doesn't
         // check that size & offsets are within bounds either for
         // that matter...please convert fonts responsibly.)
         bitPusher.align();
         int startOffset=bitPusher.offset();
         
         
         PFXglyph thisGlyph;
         thisGlyph.bitmapOffset = bitPusher.offset();
         thisGlyph.width = bitmap->width;
         thisGlyph.height = bitmap->rows;
         thisGlyph.xAdvance = face->glyph->advance.x >> 6;
         thisGlyph.xOffset = g->left;
         thisGlyph.yOffset = 1 - g->top;
         listOfGlyphs.push_back(thisGlyph);
 
         for (int y = 0; y < bitmap->rows; y++)
         {
           const uint8_t *line=bitmap->buffer+y * bitmap->pitch;
           for (int x = 0; x < bitmap->width; x++)
           {
             int byte = x / 8;
             int bit = 0x80 >> (x & 7);
             bitPusher.addBit(line[byte] & bit);
           }
         }
         bitPusher.align();
         int size=bitPusher.offset()-startOffset;
         _totalUncompressedSize+=size;
         if(compressed)
         {
             compressInPlace((uint8_t *)(bitPusher.data()+startOffset),size);
             bitPusher.setOffset(startOffset+size);
         }
     }
     face_height= face->size->metrics.height >> 6;
     FT_Done_Glyph(glyph);
     return true;
  }
 */
 
