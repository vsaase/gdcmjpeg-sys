use image::{ImageBuffer, Luma};
extern crate gdcmjpeg_sys;
use gdcmjpeg_sys::{decode12_file, decode16_file};

#[test]
fn test_decode_lossless12bit() {
    unsafe{
        let (buf, width, height) = decode12_file("data/lossless12bit.jpg");
        let img : ImageBuffer<Luma<u16>, Vec<u16>> = ImageBuffer::from_raw(width, height, buf).unwrap();
        img.save("data/test12.png").unwrap();
    }
}
#[test]
fn test_decode_lossless16bit() {
    unsafe{
        let (buf, width, height) = decode16_file("data/lossless12bit.jpg");
        let img : ImageBuffer<Luma<u16>, Vec<u16>> = ImageBuffer::from_raw(width, height, buf).unwrap();
        img.save("data/test16.png").unwrap();
    }
}