#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod bindings;

// mod bindings {
//     include!(concat!(env!("OUT_DIR"),"/bindings.rs"));
// }

use std::mem;
use std::ffi::CString;
use libc;
use bindings::{jpeg12_decompress_struct, 
    jpeg12_jpeg_CreateDecompress, 
    jpeg12_error_mgr, 
    jpeg12_jpeg_std_error,
    jpeg12_jpeg_stdio_src,
    jpeg12_jpeg_read_header,
    _IO_FILE,
    boolean,
    jpeg12_jpeg_start_decompress,
    jpeg12_jpeg_read_scanlines,
    jpeg12_jpeg_finish_decompress,
    jpeg12_jpeg_destroy_decompress,
    JSAMPARRAY
};

pub unsafe fn decode12(file_name: &str) -> (Vec<u16>, u32, u32) {
    const JPEGVERSION: ::std::os::raw::c_int = 62;
    let mut err: jpeg12_error_mgr = mem::zeroed();
    let mut cinfo: jpeg12_decompress_struct = mem::zeroed();
    cinfo.err = jpeg12_jpeg_std_error(&mut err);
    jpeg12_jpeg_CreateDecompress(&mut cinfo, JPEGVERSION, mem::size_of::<jpeg12_decompress_struct>() as u64);

    let file_name = CString::new(file_name.as_bytes()).unwrap();
    let mode = CString::new("rb").unwrap();
    let fh = libc::fopen(file_name.as_ptr(), mode.as_ptr());
    jpeg12_jpeg_stdio_src(&mut cinfo, fh as *mut _IO_FILE);
    jpeg12_jpeg_read_header(&mut cinfo, true as boolean);
    //dbg!(cinfo);
    cinfo.data_precision = 12;

    let width = cinfo.image_width;
    let height = cinfo.image_height;
    //println!("Image size {}x{}", width, height);
    
    jpeg12_jpeg_start_decompress(&mut cinfo);
    let row_stride = cinfo.image_width as usize * cinfo.output_components as usize;
    let buffer_size = row_stride * cinfo.image_height as usize;
    let mut buffer = vec![0u16; buffer_size];
    while cinfo.output_scanline < cinfo.output_height {
        let offset = cinfo.output_scanline as usize * row_stride;
        let mut jsamparray = [buffer[offset..].as_mut_ptr()];
        jpeg12_jpeg_read_scanlines(&mut cinfo, jsamparray.as_mut_ptr() as JSAMPARRAY, 1);
    }

    //println!("Decoded into {} raw pixel bytes", buffer.len());

    jpeg12_jpeg_finish_decompress(&mut cinfo);
    jpeg12_jpeg_destroy_decompress(&mut cinfo);
    libc::fclose(fh);

    (buffer, width, height)
}