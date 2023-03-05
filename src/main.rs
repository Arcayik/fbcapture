mod buffer; use crate::buffer::*;
mod image; use crate::image::*;

use std::path::Path;

use std::time::{Duration, Instant};

const CAP: usize = 1024 * 600 * 4;
// Get framebuffer data
const FILEPATH: &str = "/home/blakely/.local/cache/fbvideo/";

fn main () {
    // cmdline args
    let query = std::env::args().nth(1).expect("No arguments given");
    let project = std::env::args().nth(2).unwrap_or(String::from("tmp"));

    // check if project exists
    match Path::new(format!("{FILEPATH}/{project}").as_str()).exists() {
        true => println!("path exists"),
        false => println!("path doesn't exist")
    }

    match query.as_str() {
        "record" => record(&project),
        "compile" => compile(&project),
        _ => panic!("invalid command line argument")
    };
}

fn record(project: &String) {
    let mut total: Duration = Duration::ZERO;

    for num in 1..30 {
        let start = Instant::now();
        buffer_to_file(get_buffer("/dev/fb0"), project, num);
        let duration = start.elapsed();
        total += duration;

        //let frametime = 100/duration.as_millis();
        let framerate = total.as_millis()/num as u128;
        println!("avg: {}", framerate);
        //num += 1;
    }
}

fn compile(project: &String) {
    for num in 1..30 {
        let mut buffer = get_buffer(format!("{FILEPATH}/default/{num}").as_str());
        write_image(convert_pixels(&mut buffer), num);
    }
}

