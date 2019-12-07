use crate::bindings::{
    detection, do_nms_sort, free_network, get_network_boxes, load_network, network,
    network_predict, set_batch_network,
};
use crate::image::Image;
use std::ffi::CString;
use std::os::raw::c_char;

#[allow(dead_code)]
pub struct Network {
    net: *mut network,
    pub height: i32,
    pub width: i32,
}

impl Network {
    pub fn load(config: &str, weights: &str, clear: i32) -> Self {
        let config = CString::new(config).expect("error saving image").into_raw();
        let weight = CString::new(weights)
            .expect("error saving image")
            .into_raw();

        let net = unsafe { load_network(config as *mut c_char, weight as *mut c_char, clear) };
        Network {
            net,
            height: unsafe { (*net).h },
            width: unsafe { (*net).w },
        }
    }

    pub fn set_batch(&mut self, batch_size: i32) {
        unsafe { set_batch_network(self.net, batch_size) };
    }

    /// this funtion normally returns a pointer to the output of the last layer, but this
    /// pointer may be dangling in the future, so we dont.
    pub fn predict_image(&self, image: &Image) {
        unsafe { network_predict(self.net, image.image.data) };
    }

    pub fn get_detections(&mut self, image: &Image, thresh: f32, hier: f32) -> Vec<detection> {
        let mut n_boxes = 0;
        let n_boxes_raw = &mut n_boxes as *mut i32;
        let dets = unsafe {
            get_network_boxes(
                self.net,
                image.image.w,
                image.image.h,
                thresh,
                hier,
                // dunno what these do in the original code, replace by values found in example...
                0 as *mut i32,
                1,
                n_boxes_raw,
            )
        };
        println!("here");
        // ownership on this memory segment is taken here, drop will free each pieces when it gets
        // out of scope
        unsafe { Vec::from_raw_parts(dets, (*n_boxes_raw) as usize, n_boxes as usize) }
    }
}

pub trait DetecVec {
    fn do_nms_sort(&mut self, classes: i32, NMS: f32);
}

impl DetecVec for Vec<detection> {
    fn do_nms_sort(&mut self, classes: i32, NMS: f32) {
        unsafe { do_nms_sort(self.as_mut_ptr(), self.len() as i32, classes, NMS) };
    }
}

impl Drop for Network {
    fn drop(&mut self) {
        unsafe {
            free_network(self.net);
        }
    }
}

// can't use the free_detections from original lib, as we are passing around a vec instead of a
// pointer
impl Drop for detection {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.prob as *mut std::ffi::c_void);
            if self.mask != std::ptr::null_mut() {
                libc::free(self.mask as *mut std::ffi::c_void)
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_load_network() {
        let _net = Network::load(
            "/home/mpostma/Documents/code/rust/darnet-sys/yolov2-tiny.cfg",
            "/home/mpostma/Documents/code/rust/darnet-sys/yolov2-tiny.weights",
            0,
        );
    }
}
