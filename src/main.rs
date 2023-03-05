mod buffer; use crate::buffer::*;
mod image; use crate::image::*;

use std::path::Path;
use std::time::{Duration, Instant};

const CAP: usize = 1024 * 600 * 4;
// Get framebuffer data
const FILEPATH: &str = "/home/blakely/.local/cache/fbvideo/";

fn main () {
    // cmdline args
    let query = std::env::args().nth(1)
        .expect("No arguments given");
    let directory = std::env::args().nth(2)
        .unwrap_or(String::from(FILEPATH));

    // check if directory exists
    match Path::new(&directory).exists() {
        true => println!("path exists"),
        false => println!("path doesn't exist")
    }

    match query.as_str() {
        "record" => record(&directory),
        "compile" => compile(&directory),
        _ => panic!("invalid command line argument")
    };
}

fn record(directory: &String) {
    let mut total: Duration = Duration::ZERO;

    for num in 1..30 {
        let start = Instant::now();
        buffer_to_file(get_buffer("/dev/fb0"), directory, num);
        let duration = start.elapsed();
        total += duration;

        //let frametime = 100/duration.as_millis();
        let framerate = total.as_millis()/num as u128;
        println!("avg: {}", framerate);
        //num += 1;
    }
}

fn compile(directory: &String) {
    for num in 1..30 {
        let mut buffer = get_buffer(format!("{FILEPATH}/{directory}/{num}").as_str());
        write_image(convert_pixels(&mut buffer), num);
    }
}

