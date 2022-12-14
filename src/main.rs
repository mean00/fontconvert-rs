#![allow(non_snake_case)]
#![feature(path_file_prefix)]
use clap;
use clap::Parser;
use std::fs::File;
use std::io::Write;
use std::path::Path;
extern crate freetype as ft;
extern crate heatshrink_byte as hs;


mod engine;
mod bit_pusher;


#[derive(Parser, Debug)]
#[clap(author="mean00",version="0.1",about="Enhanced TTF to GFX converter",long_about = None)]
struct Config {
    /// Path to the font to use
    #[clap(short, long)]
    font: String,
    /// output file (C header)
    #[clap(short, long)]
    output_file: Option<String>,
    /// output file (rust header)
    #[clap(short='r', long)]
    output_file_rs: Option<String>,
    /// bitmap file
    #[clap(short='m', long, default_value = "")]
    bitmap_file: String,
    #[clap(short, long)]
    /// Size of the font to render
    size: usize,
    /// Ascii value of the first char to render
    #[clap(short, long, default_value = "32")]    
    begin: u8,
    /// Ascii value of the last char to render
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
fn print_output_header(mut out:&File,_args:&Config) -> Result< () ,std::io::Error> {
    
    out.write_all(b"// Generated by flatconvert-rs  https://github.com/mean00/flatconvert-rs.git \n")?;
    out.write_all(b"// a modified version of adafruit fontconvert \n")?;
    out.write_all(b"#pragma once\n")?;
    
    Ok(())
}
fn print_output_header_rs(mut out:&File,_args:&Config) -> Result< () ,std::io::Error> {
    
    out.write_all(b"// Generated by flatconvert-rs  https://github.com/mean00/flatconvert-rs.git \n")?;
    out.write_all(b"// a modified version of adafruit fontconvert \n")?;
    out.write_all(b"// RUST HEADER\n")?;
    out.write_all(b"#![allow(non_upper_case_globals)]\n")?;
    out.write_all(b"use simpler_gfx::PFXglyph;\n")?;
    out.write_all(b"use simpler_gfx::PFXfont;\n")?;
    Ok(())
}


fn write_file(engine : &mut engine::Engine, out : &mut File, symbol : &String ) -> Result< () ,std::io::Error>
{    
    engine.dump_bitmap( out, &symbol)?;
    engine.dump_index( out, &symbol)?;
    engine.dump_footer( out, &symbol)?;
    Ok(())
}

fn write_file_rs(engine : &mut engine::Engine, out : &mut File, symbol : &String ) -> Result< () ,std::io::Error>
{    
    engine.dump_rs( out, &symbol)?;
    Ok(())
}


/**
 * 
 */
fn main() {
    let args=Config::parse();
    if let Some(ref name) = args.output_file
    {
        println!("Font : {} ==> C file {} ", args.font,name);
    }
    if let Some(ref name) = args.output_file_rs
    {
        println!("Font : {} ==> Rust file {} ", args.font,name);
    }
    
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
            mapp[i as usize]=1;
        }    
    }else
    {
        for i in args.begin..=args.end
        {
            mapp[i as usize]=1;
        }
    }
    let mut engine = engine::Engine::new( &args.font, args.size).expect("Failure to initialize engine");
    engine.run(args.bpp , 
        args.compression, 
        &mapp)  .expect("Failed to render fonts");
    
   
    let mut symbol_name : String ;
    
    let path = Path::new(&args.font);
    symbol_name = String::from(path.file_prefix().unwrap().to_str().unwrap());
    symbol_name = symbol_name.replace(" ","_");
    symbol_name = symbol_name.replace("-","_");
    let extension = args.size.to_string()+"pt7b";     // [size]pt[7]
    symbol_name.push_str(&extension);


    // output C file
    if let Some(ref name) = args.output_file
    {
        let mut ofile = File::create(name.clone()).expect("unable to create file");        
        match print_output_header(&ofile,&args)
        {
            Ok(_x) => (),
            Err(_x) => panic!("Cannot create output file"),
        }
        match write_file(&mut engine , &mut ofile, &symbol_name)
        {
            Ok(_x) => (),
            Err(_x) => panic!("Cannot create output file"),
        }
        drop(ofile); // make sure it's closed
    }
    if let Some(ref name) = args.output_file_rs
    {
        let mut ofile = File::create(name).expect("unable to create rs file");        
        match print_output_header_rs(&ofile,&args)
        {
            Ok(_x) => (),
            Err(_x) => panic!("Cannot create output file"),
        }
        match write_file_rs(&mut engine , &mut ofile, &symbol_name)
        {
            Ok(_x) => (),
            Err(_x) => panic!("Cannot create output file"),
        }
        drop(ofile); // make sure it's closed
    }


    println!("-Done-") 
}
//--eof--
