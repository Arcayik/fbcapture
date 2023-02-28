mod buffer; use crate::buffer::*;
mod image; use crate::image::*;

use std::time::{Duration, Instant};

const CAP: usize = 1024 * 600 * 4;
const FILEPATH: &str = "/home/blakely/.local/cache/fbvideo/";

fn main () {
    let mut total: Duration = Duration::ZERO;

    for num in 1..30 {
        let start = Instant::now();
        buffer_to_file(get_buffer("/dev/fb0"), num);
        let duration = start.elapsed();
        total += duration;

        //let frametime = 100/duration.as_millis();
        let framerate = total.as_millis()/num as u128;
        println!("avg: {}", framerate);
        //num += 1;
    }

    for num in 1..30 {
        let mut buffer = get_buffer(format!("{FILEPATH}/default/{num}").as_str());
        
        image(convert_pixels(&mut buffer), num);
    }
    //println!("{:?}", total);
}
// Take command line arguments on whether to record or compile
