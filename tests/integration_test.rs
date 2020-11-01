use image::{ImageBuffer, Luma};
extern crate gdcmjpeg_sys;
use gdcmjpeg_sys::decode12;

#[test]
fn test_decode_lossless12bit() {
    unsafe{
        let (buf, width, height) = decode12("data/lossless12bit.jpg");
        let img : ImageBuffer<Luma<u16>, Vec<u16>> = ImageBuffer::from_raw(width, height, buf).unwrap();
        img.save("data/test.png").unwrap();
    }
}