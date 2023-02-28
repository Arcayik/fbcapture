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

pub fn buffer_to_file(buf: [u8;CAP], file: u32)
{
    File::create(format!("{FILEPATH}/default/{file}")).unwrap()
        .write_all(&buf).unwrap();
}
