#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

pub mod bindings;
pub mod image;
pub mod network;

#[cfg(test)]
mod test {

    use super::bindings::*;
    use super::image::*;
    use super::network::DetecVec;
    use super::network::Network;

    #[test]
    fn test_lib_rust() {
        let voc_names: Vec<&'static str> = vec![
            "person",
            "bicycle",
            "car",
            "motorbike",
            "aeroplane",
            "bus",
            "train",
            "truck",
            "boat",
            "traffic light",
            "fire hydrant",
            "stop sign",
            "parking meter",
            "bench",
            "bird",
            "cat",
            "dog",
            "horse",
            "sheep",
            "cow",
            "elephant",
            "bear",
            "zebra",
            "giraffe",
            "backpack",
            "umbrella",
            "handbag",
            "tie",
            "suitcase",
            "frisbee",
            "skis",
            "snowboard",
            "sports ball",
            "kite",
            "baseball bat",
            "baseball glove",
            "skateboard",
            "surfboard",
            "tennis racket",
            "bottle",
            "wine glass",
            "cup",
            "fork",
            "knife",
            "spoon",
            "bowl",
            "banana",
            "apple",
            "sandwich",
            "orange",
            "broccoli",
            "carrot",
            "hot dog",
            "pizza",
            "donut",
            "cake",
            "chair",
            "sofa",
            "pottedplant",
            "bed",
            "diningtable",
            "toilet",
            "tvmonitor",
            "laptop",
            "mouse",
            "remote",
            "keyboard",
            "cell phone",
            "microwave",
            "oven",
            "toaster",
            "sink",
            "refrigerator",
            "book",
            "clock",
            "vase",
            "scissors",
            "teddy bear",
            "hair drier",
            "toothbrush",
        ];
        const THRESH: f32 = 0.5;
        const NMS: f32 = 0.45;
        const HIER_THRES: f32 = 0.5;

        let mut net = Network::load("yolov3.cfg", "yolov3.weights", 0);
        net.set_batch(1);
        unsafe { srand(22222) }
        let mut image = Image::load_color("test.jpg", 0, 0);
        let image_resized = image.resize_to_letterbox(net.width, net.height);
        image_resized.save("testest");
        net.predict_image(&image_resized);
        println!("here");
        let mut dets = net.get_detections(&image, THRESH, HIER_THRES);
        println!("found {} boxes", dets.len());
        dets.do_nms_sort(80, NMS);
        image.draw_detection(&mut dets, THRESH, &voc_names);
        image.save("woot");
    }
}
