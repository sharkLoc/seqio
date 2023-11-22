//! # File io utils
//! * [`file_reader`] open uncompressed/gzipped file from `std::io::stdin` or file
//! * [`file_writer`] write uncompressed/gzipped content to `std::io::stdout` or file
//! * [`file_writer_append`] append uncompressed/gzipped content to file
//! 
//! # Example
//! The following example shows how to use.
//! 
//! ### read: [`file_reader`]
//! ```rust ignore
//! use std::io::BufRead;
//! use seqio::utils::file_reader;
//! 
//! let file: &Option<&str> = &Some("path_to_file"); // for uncompressed file
//! let file: &Option<&str> = &Some("path_to_file.gz"); // for gzipped compressed file
//! let file: &Option<&str> = &None; // read from std::io::stdin
//! 
//! let fp = file_reader(file)?;
//! 
//! for line in fp.lines().flatten() {
//!     println!("{}",line);
//!     todo!();
//! }
//! ```
//! 
//! 
//! ### write: [`file_writer`]
//! 
//! ```rust ignore
//! use std::io::BufRead;
//! use seqio::utils::file_writer;
//! 
//! let output: &Option<&str> = &Some("write_file_name"); // output uncompressed file
//! let output: &Option<&str> = &Some("write_file_name.gz"); // output gzipped compressed file
//! let output: &Option<&str> = &None; //  write to std::io::stdout
//! 
//! let mut fo = file_writer(output)?;
//! fo.write(b"foo\nbar")?;
//! fo.flush()?;
//! ```
//! 
//! 
//! ### append: [`file_writer_append`]
//! ```rust ignore
//! use std::io::BufRead;
//! use seqio::utils::file_writer_appned;
//! 
//! let append: &str = "newfile";
//! let append: &str = "append_file.gz";
//! 
//! let mut fo_append = file_writer_append(append)?;
//! 
//! fo_append.write(b"line one")?;
//! fo_append.write(b"line two")?;
//! fo.flush()?
//! ```
//! Automatically create new files if file is not exists
/// date 2023.11.23



use std::{
    fs::{File, OpenOptions},
    io::{self, prelude::*, BufRead, BufReader, BufWriter, Result, Write},
};
use flate2::{read, write, Compression};

/// gzipped file magic number
pub const GZ_MAGIC: [u8; 3] = [0x1f, 0x8b, 0x08];

/// io buffer size
pub const BUFF_SIZE: usize = 1024 * 1024;

/// file is gzipped or not
pub fn is_gzipped(file_name: &str) -> Result<bool> {
    let mut buffer: [u8; 3] = [0; 3];
    let mut fp = File::open(file_name)?;
    let _x = fp.read(&mut buffer)?;
    Ok(buffer == GZ_MAGIC || file_name.ends_with(".gz"))
}

/// open a file reader and return a iterator
pub fn file_reader(file_in: &Option<&str>) -> Result<Box<dyn BufRead>> {
    if let Some(file_name) = file_in {
        let fp = File::open(file_name)?;
        let flag = is_gzipped(file_name)?;

        if flag {
            Ok(Box::new(BufReader::with_capacity(
                BUFF_SIZE,
                read::MultiGzDecoder::new(fp),
            )))
        } else {
            Ok(Box::new(BufReader::with_capacity(BUFF_SIZE, fp)))
        }
    } else {
        let fp = BufReader::new(io::stdin());
        Ok(Box::new(fp))
    }
}

/// open a file and return a file writer
pub fn file_writer(file_out: &Option<&str>) -> Result<Box<dyn Write>> {
    if let Some(file_name) = file_out {
        let fp = File::create(file_name)?;
        if file_name.ends_with(".gz") || file_name.ends_with(".gzip") {
            Ok(Box::new(BufWriter::with_capacity(
                BUFF_SIZE,
                write::GzEncoder::new(fp, Compression::default()),
            )))
        } else {
            Ok(Box::new(BufWriter::with_capacity(BUFF_SIZE, fp)))
        }
    } else {
        Ok(Box::new(BufWriter::new(io::stdout())))
    }
}

/// open a file and return a file writer with append model
pub fn file_writer_append(file_out: &str) -> Result<Box<dyn Write>> {
    let fp = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_out)?;
    
    if file_out.ends_with(".gz") || file_out.ends_with(".gzip") {
        Ok(Box::new(BufWriter::with_capacity(
            BUFF_SIZE,
            write::GzEncoder::new(fp, Compression::default()),
        )))
    } else {
        Ok(Box::new(BufWriter::with_capacity(BUFF_SIZE, fp)))    
    }
}
