
use clap;
use clap::Parser;
extern crate freetype as ft;


#[derive(Parser, Debug)]
#[clap(author="mean00",version="0.1",about="Enhanced TTF to GFX converter",long_about = None)]
struct Config {
    /// Path to the font to use
    #[clap(short, long)]
    font: String,
    /// output file (C header)
    #[clap(short, long)]
    output_file: String,
    /// bitmap file
    #[clap(short='m', long, default_value = "")]
    bitmap_file: String,
    /// Size of the font to render
    #[clap(short, long)]
    size: u8,
    /// Ascii value of the first char to render
    #[clap(short, long, default_value = "32")]
    /// Ascii value of the last char to render
    begin: u8,
    #[clap(short, long, default_value = "127")]
    end: u8,
    /// bpp  (1= B&W, 2=4 levels of grey, 4 = 16 levels of grey)
    #[clap(short='p', long, default_value = "1")]
    bpp: u8,
    /// compression
    #[clap(short, long)]
    compression: bool,
}

//------------------
fn main() {
        let args=Config::parse();
        println!("Font : {} ==> {} ", args.font,args.output_file);
        println!("Size : {}", args.size);
        println!("Range: {} to  {}", args.begin, args.end);
        println!("bpp  : {}", args.bpp);
        println!("Comp : {}", args.compression);
        //--
        let library = ft::Library::init().unwrap();
        let face = library.new_face(args.font, 0).unwrap();
        println!("--") 
}
