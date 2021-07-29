#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod bindings8;
pub mod bindings12;
pub mod bindings16;

use std::mem;
use std::ffi::CString;
use libc;
use bindings12::{jpeg12_decompress_struct, 
    jpeg12_jpeg_CreateDecompress, 
    jpeg12_error_mgr, 
    jpeg12_jpeg_std_error,
    jpeg12_jpeg_stdio_src,
    jpeg12_jpeg_read_header,
    boolean,
    jpeg12_jpeg_start_decompress,
    jpeg12_jpeg_read_scanlines,
    jpeg12_jpeg_finish_decompress,
    jpeg12_jpeg_destroy_decompress,
};
use bindings16::{
    jpeg16_decompress_struct, 
    jpeg16_jpeg_CreateDecompress, 
    jpeg16_error_mgr, 
    jpeg16_jpeg_std_error,
    jpeg16_jpeg_stdio_src,
    jpeg16_jpeg_read_header,
    jpeg16_jpeg_start_decompress,
    jpeg16_jpeg_read_scanlines,
    jpeg16_jpeg_finish_decompress,
    jpeg16_jpeg_destroy_decompress,
};

pub fn decode12_bytes(src: &mut [u8]) -> (Vec<u16>, u32, u32) {
    unsafe {
        let srclen = src.len() as libc::size_t;
        let psrc = src.as_mut_ptr() as *mut libc::c_void;


        const JPEGVERSION: ::std::os::raw::c_int = 62;
        let mut err: jpeg12_error_mgr = mem::zeroed();
        let mut cinfo: jpeg12_decompress_struct = mem::zeroed();
        cinfo.err = jpeg12_jpeg_std_error(&mut err);
        jpeg12_jpeg_CreateDecompress(&mut cinfo, JPEGVERSION, mem::size_of::<jpeg12_decompress_struct>() as u64);
        let mode = CString::new("rb").unwrap();
        let fh = libc::fmemopen(psrc, srclen, mode.as_ptr());
        jpeg12_jpeg_stdio_src(&mut cinfo, fh as *mut bindings12::_IO_FILE);
        jpeg12_jpeg_read_header(&mut cinfo, true as boolean);
        //dbg!(cinfo);
    
        let width = cinfo.image_width;
        let height = cinfo.image_height;

        jpeg12_jpeg_start_decompress(&mut cinfo);
        let row_stride = cinfo.image_width as usize * cinfo.output_components as usize;
        let buffer_size = row_stride * cinfo.image_height as usize;
        let mut buffer = vec![0u16; buffer_size];
        while cinfo.output_scanline < cinfo.output_height {
            let offset = cinfo.output_scanline as usize * row_stride;
            let mut jsamparray = [buffer[offset..].as_mut_ptr()];
            jpeg12_jpeg_read_scanlines(&mut cinfo, jsamparray.as_mut_ptr() as bindings12::JSAMPARRAY, 1);
        }
        jpeg12_jpeg_finish_decompress(&mut cinfo);
        jpeg12_jpeg_destroy_decompress(&mut cinfo);
        libc::fclose(fh);

        (buffer, width, height)
    }
}

pub unsafe fn decode12_file(file_name: &str) -> (Vec<u16>, u32, u32) {
    const JPEGVERSION: ::std::os::raw::c_int = 62;
    let mut err: jpeg12_error_mgr = mem::zeroed();
    let mut cinfo: jpeg12_decompress_struct = mem::zeroed();
    cinfo.err = jpeg12_jpeg_std_error(&mut err);
    jpeg12_jpeg_CreateDecompress(&mut cinfo, JPEGVERSION, mem::size_of::<jpeg12_decompress_struct>() as u64);

    let file_name = CString::new(file_name.as_bytes()).unwrap();
    let mode = CString::new("rb").unwrap();
    let fh = libc::fopen(file_name.as_ptr(), mode.as_ptr());
    jpeg12_jpeg_stdio_src(&mut cinfo, fh as *mut bindings12::_IO_FILE);
    jpeg12_jpeg_read_header(&mut cinfo, true as boolean);
    dbg!(cinfo);

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
        jpeg12_jpeg_read_scanlines(&mut cinfo, jsamparray.as_mut_ptr() as bindings12::JSAMPARRAY, 1);
    }

    //println!("Decoded into {} raw pixel bytes", buffer.len());

    jpeg12_jpeg_finish_decompress(&mut cinfo);
    jpeg12_jpeg_destroy_decompress(&mut cinfo);
    libc::fclose(fh);

    (buffer, width, height)
}


pub fn decode16_bytes(src: &mut [u8]) -> (Vec<u16>, u32, u32) {
    unsafe {
        let srclen = src.len() as libc::size_t;
        let psrc = src.as_mut_ptr() as *mut libc::c_void;


        const JPEGVERSION: ::std::os::raw::c_int = 62;
        let mut err: jpeg16_error_mgr = mem::zeroed();
        let mut cinfo: jpeg16_decompress_struct = mem::zeroed();
        cinfo.err = jpeg16_jpeg_std_error(&mut err);
        jpeg16_jpeg_CreateDecompress(&mut cinfo, JPEGVERSION, mem::size_of::<jpeg16_decompress_struct>() as u64);
        let mode = CString::new("rb").unwrap();
        let fh = libc::fmemopen(psrc, srclen, mode.as_ptr());
        jpeg16_jpeg_stdio_src(&mut cinfo, fh as *mut bindings16::_IO_FILE);
        jpeg16_jpeg_read_header(&mut cinfo, true as boolean);
        //dbg!(cinfo);
    
        let width = cinfo.image_width;
        let height = cinfo.image_height;

        jpeg16_jpeg_start_decompress(&mut cinfo);
        let row_stride = cinfo.image_width as usize * cinfo.output_components as usize;
        let buffer_size = row_stride * cinfo.image_height as usize;
        let mut buffer = vec![0u16; buffer_size];
        while cinfo.output_scanline < cinfo.output_height {
            let offset = cinfo.output_scanline as usize * row_stride;
            let mut jsamparray = [buffer[offset..].as_mut_ptr()];
            jpeg16_jpeg_read_scanlines(&mut cinfo, jsamparray.as_mut_ptr() as bindings16::JSAMPARRAY, 1);
        }
        jpeg16_jpeg_finish_decompress(&mut cinfo);
        jpeg16_jpeg_destroy_decompress(&mut cinfo);
        libc::fclose(fh);

        (buffer, width, height)
    }
}

pub unsafe fn decode16_file(file_name: &str) -> (Vec<u16>, u32, u32) {
    const JPEGVERSION: ::std::os::raw::c_int = 62;
    let mut err: jpeg16_error_mgr = mem::zeroed();
    let mut cinfo: jpeg16_decompress_struct = mem::zeroed();
    cinfo.err = jpeg16_jpeg_std_error(&mut err);
    jpeg16_jpeg_CreateDecompress(&mut cinfo, JPEGVERSION, mem::size_of::<jpeg16_decompress_struct>() as u64);

    let file_name = CString::new(file_name.as_bytes()).unwrap();
    let mode = CString::new("rb").unwrap();
    let fh = libc::fopen(file_name.as_ptr(), mode.as_ptr());
    jpeg16_jpeg_stdio_src(&mut cinfo, fh as *mut bindings16::_IO_FILE);
    jpeg16_jpeg_read_header(&mut cinfo, true as boolean);
    dbg!(cinfo);

    let width = cinfo.image_width;
    let height = cinfo.image_height;
    //println!("Image size {}x{}", width, height);
    
    jpeg16_jpeg_start_decompress(&mut cinfo);
    let row_stride = cinfo.image_width as usize * cinfo.output_components as usize;
    let buffer_size = row_stride * cinfo.image_height as usize;
    let mut buffer = vec![0u16; buffer_size];
    while cinfo.output_scanline < cinfo.output_height {
        let offset = cinfo.output_scanline as usize * row_stride;
        let mut jsamparray = [buffer[offset..].as_mut_ptr()];
        jpeg16_jpeg_read_scanlines(&mut cinfo, jsamparray.as_mut_ptr() as bindings16::JSAMPARRAY, 1);
    }

    //println!("Decoded into {} raw pixel bytes", buffer.len());

    jpeg16_jpeg_finish_decompress(&mut cinfo);
    jpeg16_jpeg_destroy_decompress(&mut cinfo);
    libc::fclose(fh);

    (buffer, width, height)
}