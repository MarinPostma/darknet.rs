#![allow(dead_code)]

use crate::bindings::{
    detection, draw_detections, free_image, image, letterbox_image, load_alphabet,
    load_image_color, save_image,
};
use std::ffi::CString;
use std::mem;
use std::os::raw::c_char;

pub struct Image {
    pub image: image,
    pub width: i32,
    pub height: i32,
}

impl Image {
    /// Loads rgb image and returns a image object. if width and height are set to 0, the height
    /// and width are infered.
    pub fn load_color(filename: &str, width: i32, height: i32) -> Self {
        let string = CString::new(filename)
            .expect("error saving image")
            .into_raw();
        let image = unsafe { load_image_color(string, width, height) };
        Self {
            image,
            width: image.w,
            height: image.h,
        }
    }

    pub fn new(width: i32, height: i32, colors: i32, data: &[f32]) -> Self {
        let mut data = Vec::from(data);
        data.shrink_to_fit();
        let image = image {
            w: width.into(),
            h: height.into(),
            c: colors.into(),
            data: data.as_mut_ptr(),
        };

        mem::forget(data);
        Self {
            image,
            width,
            height,
        }
    }

    pub fn save(&self, name: &str) {
        let string = CString::new(name).expect("error saving image");
        let bytes = string.as_bytes_with_nul();
        let ptr = bytes.as_ptr();
        unsafe {
            save_image(self.image, ptr as *const c_char);
        }
    }

    pub fn resize_to_letterbox(&self, width: i32, height: i32) -> Self {
        let image = unsafe { letterbox_image(self.image, width, height) };
        Self {
            image,
            width: image.w,
            height: image.h,
        }
    }

    pub fn draw_detection(&mut self, dets: &mut [detection], tresh: f32, class_names: &[&str]) {
        //this function is potentially teribbly unsafe
        println!("here");
        let alphabet = unsafe { load_alphabet() };
        let mut names = class_names
            .iter()
            .map(|name| CString::new(*name).unwrap().into_raw())
            .collect::<Vec<*mut c_char>>();
        unsafe {
            draw_detections(
                self.image,
                dets.as_mut_ptr(),
                dets.len() as i32,
                tresh,
                names.as_mut_ptr() as *mut *mut c_char,
                alphabet,
                class_names.len() as i32,
            );
        }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { free_image(self.image) }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn load_image() {
        let _image = Image::load_color("test.jpg", 0, 0);
    }
    #[test]
    fn allocate_and_free_image() {
        let vals = [1.0; 10000];
        let _ = Image::new(10, 10, 2, &vals);
    }

    #[test]
    fn test_save_file() {
        let vals = [1.0; 10000];
        let img = Image::new(10, 10, 2, &vals);
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_str().unwrap();
        img.save(&path);
    }
}
