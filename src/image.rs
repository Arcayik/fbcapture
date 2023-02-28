use crate::{CAP, FILEPATH};
use crate::buffer::get_buffer;

pub fn convert_pixels(buf: &mut [u8;CAP]) -> [u8;CAP]
{
    for num in (0..CAP).step_by(4) {
        // blue and red are swapped for some reason.
        buf[num+3] = buf[num+2]; // Alpha <- Blue
        buf[num+2] = buf[num]; // Blue <- Red
        buf[num] = buf[num+3]; // Red <- Alpha(Blue)
        buf[num+3] = 255; // full opacity

        //println!("r:{}, g:{}, b:{}, a:{}",
        //    buf[num], buf[num+1], buf[num+2], buf[num+3]);
    }
    *buf
}

pub fn image(buf: [u8;CAP], file: i32)
{
    image::save_buffer(format!("{FILEPATH}/default/img/{file}.png"), &buf,
        1024, 600, image::ColorType::Rgba8)
        .unwrap();
}
