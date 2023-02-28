use crate::{CAP, FILEPATH};
use std::fs::File;
use std::io::prelude::*;

pub fn get_buffer(file: &str) -> [u8;CAP]
{
    let mut buf = [0u8;CAP];
    File::open(file).unwrap()
        .read(&mut buf).unwrap();
    buf
}

pub fn buffer_to_file(buf: [u8;CAP], project: &String, file: u32)
{
    // Check that project dir exists
    File::create(format!("{FILEPATH}/{project}/{file}"))
        .expect("(buffer_to_file) File not found")
        .write_all(&buf).unwrap();
}
