use std::io;
use std::io::prelude::*;
use std::fs::File;

use clap::Parser;

fn byte_to_ascii(byte: &u8) -> Option<char>{
    match byte{
        32..=126 => Some(*byte as char),
        _ => None
    }
}

fn print_index(start: usize, ending: usize, width: usize){
    print!("[{:w$}-{:w$}] ", start, ending, w=width);
}

fn print_hex(buffer: &[u8], max_bytes: usize){
    for byte in buffer{
        print!("{:02X} ", byte)
    }

    for _ in buffer.len()..max_bytes{
        print!("   ");
    }
}

fn print_ascii(buffer: &[u8], max_bytes: usize){
    for byte in buffer{
        let ascii = byte_to_ascii(byte);
        let out = match ascii{
            None => '.',
            Some(ascii) => ascii,
        };
        print!("{out}")
    }

    for _ in buffer.len()..max_bytes{
        print!(" ");
    }
}

fn print_line(i: usize, buffer: &[u8], max_bytes: usize, width: usize){
    print_index(i*max_bytes, i*max_bytes + buffer.len()-1, width);
    print_hex(buffer, max_bytes);
    print!("  ");
    print_ascii(buffer, max_bytes);
    print!("\n");
}

fn load_binary_file(filename: String, bytes_per_line: usize)  -> io::Result<()> {
    let mut f = File::open(filename)?;
    let file_size = f.metadata().unwrap().len();
    let width = (file_size as f64).log10().ceil() as usize;
    println!("filesize: {file_size} {width}");
    let mut buffer : Vec<u8> = vec![0; bytes_per_line];

    for i in 0..(file_size as f64/ bytes_per_line as f64).ceil() as usize {
        let n = f.read(&mut buffer[..])?;
        print_line(i,&buffer[0..n], bytes_per_line, width);
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, long_about=None)]
struct Args{
    #[arg(short, long, help="Binary file to be heard")]
    input_file: String,

    #[arg(short, long, help="File for hex output")]
    output_file: Option<String>,

    #[arg(short, long, help="Bytes per line", default_value_t=16)]
    bytes_per_line: usize,
}

fn main() {
    let args = Args::parse();

    println!("{:?}",args);
    load_binary_file(args.input_file, args.bytes_per_line).unwrap();
}
