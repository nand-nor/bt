use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write, Seek, SeekFrom};
use std::process;


fn main() {
    //check valid number of args provided
    if env::args().len() != 4 && env::args().len() !=5 {
        display_useage();
        process::exit(1);
    }

    let infile = env::args().nth(1).unwrap();
    let outfile = env::args().nth(2).unwrap();
    let offset= check_offset();
    let size: Option<usize> = check_size_option();
    let mut inbuf = vec![];
    read_bytes(infile, &mut inbuf, offset, size);
    write_bytes(outfile, &inbuf);
    process::exit(0);
}

/*Display useage options. Size arg is optional*/
fn display_useage(){
    println!("Useage: {:?} <infile> <outfile> <byte offset> [<size>]",
             env::args().nth(0).unwrap());
}

/*
* If a size argument is provided, check that is is a valid base 10 number and return it. If not,
* exit
*/
fn check_size_option()->Option<usize>{
    if env::args().len() == 5 {
        let ssize = env::args().nth(4).unwrap();
        let t_size = usize::from_str_radix(&ssize, 10);
        if t_size.is_err(){
            println!("BTRIMMER ERROR: invalid value for size {:?} provided",
                     ssize);
            process::exit(1);
        }
        return Some(t_size.unwrap())
    } else {
        return None
    }
}

/*
* pull the offset arg, ensure it is a valid base 10 number, and return it. Otherwise exit
*/
fn check_offset()->usize{
    let soffset = env::args().nth(3).unwrap();
    let offset = usize::from_str_radix(&soffset, 10);
    if offset.is_err() {
        println!("BTRIMMER ERROR: invalid value for offset {:?} provided",
                 soffset);
        process::exit(1);
    }
    offset.unwrap()
}

/*
* Open the infile, seek to the provided offset, and if size option given, read
* that many bytes. If not, read from offset to end of file. Write values into the
* inbuf vector
*/
fn read_bytes(infile: String, inbuf: &mut Vec<u8>, offset: usize, size: Option<usize>) {
    let mut in_fp = match File::open(infile.clone()) {
        Err(why) => {
            println!("BTRIMMER ERROR: Could not open target infile: {}: {}",
                     infile, why.description());
            process::exit(1);
        },
        Ok(fp) => fp,
    };

    match in_fp.seek(SeekFrom::Start(offset as u64)){
        Err(why) => {
            println!("BTRIMMER ERROR: Could not seek to provided offset {} of infile: {}: {}",
                     offset, infile, why.description());
            process::exit(1);
        }
        Ok(_) => {}
    }

    if let Some(size) = size {
        inbuf.resize(size, 0u8);
        match in_fp.read_exact(inbuf) {
            Err(why) => {
                println!("BTRIMMER ERROR: Could not read {} bytes at offset {} of infile: {}: {}",
                         size, offset, infile, why.description());
                process::exit(1);
            }
            Ok(_) => {}
        }
    } else {
        match in_fp.read_to_end(inbuf) {
            Err(why) => {
                println!("BTRIMMER ERROR: Could not read bytes of infile: {}: {}",
                         infile, why.description());
                process::exit(1);
            }
            Ok(_) => {}
        }
    }
}

/*
* Once byte slice has been read, open or create the requested outfile name,
* and write the copied bytes to the file
*/
fn write_bytes(outfile: String, inbuf: &Vec<u8>){

    let mut dir_path = std::env::current_dir().unwrap();
    dir_path.push(outfile.as_str());

    let mut out_fp = match File::create(dir_path.as_path()) {
        Err(why) => {
            println!("BTRIMMER ERROR: Could not open target output file: {}: {}",
                     dir_path.as_path().display(), why.description());
            process::exit(1);
        },
        Ok(fp) => fp,
    };

    match out_fp.write_all(&inbuf.as_slice()){
        Err(why) => {
            println!("BTRIMMER ERROR: Could not write to target output file: {}: {}",
                     dir_path.as_path().display(), why.description());
            process::exit(1);
        },
        Ok(()) => {}
    }
}
