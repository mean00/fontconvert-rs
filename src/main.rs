
use clap;
use clap::Parser;
use std::fs::File;
use std::io::Write;
extern crate freetype as ft;

mod engine;

const DPI:u32 = 141;

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
    size: usize,
    /// Ascii value of the first char to render
    #[clap(short, long, default_value = "32")]
    /// Ascii value of the last char to render
    begin: u8,
    #[clap(short, long, default_value = "127")]
    end: u8,
    /// bpp  (1= B&W, 2=4 levels of grey, 4 = 16 levels of grey)
    #[clap(short='p', long, default_value = "1")]
    bpp: u8,
    #[clap(short='k', long, default_value_t = String::from(""))]
    /// String to pick individual chars i.e. -k "abcd" will only render a,b,c,d
    pick: String,

    /// compression
    #[clap(short, long)]
    compression: bool,
}
/**
 * 
 * 
 */
fn print_output_header(mut out:&File,_args:&Config) -> bool {
    
    out.write_all(b"// Generated by flatconvert-rs  https://github.com/mean00/flatconvert-rs.git \n");
    out.write_all(b"// a modified version of adafruit fontconvert \n");
    out.write_all(b"#pragma once\n");
    
    true
}
/**
 * 
 */
fn main() {
    let args=Config::parse();
    println!("Font : {} ==> {} ", args.font,args.output_file);
    println!("Size : {}", args.size);
    println!("Range: {} to  {}", args.begin, args.end);
    println!("bpp  : {}", args.bpp);
    println!("Comp : {}", args.compression);
    println!("Pick : {}", args.pick);
    //--
    println!("Setting up freetype...");
    
    let mut mapp : [u8;256]= [0;256];
    if args.pick.len()!=0
    {
        for i in args.pick.chars()
        {
            let c: usize = mapp[i as usize] as usize;
            mapp[c]=1;
        }    
    }else
    {
        for i in args.begin..args.end
        {
            mapp[i as usize]=1;
        }
    }
    let engine = engine::Engine::new( &args.font, args.size, args.bpp,mapp).expect("Failure to initialize engine");
      
    let ofile = File::create(args.output_file.clone()).expect("unable to create file");
    print_output_header(&ofile,&args);



    println!("-Done-") 
}
//--eof--