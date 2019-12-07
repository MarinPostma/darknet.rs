use procinfo::pid::statm_self;
use darknet::image::*;

#[test]
fn leak_test() {
    let vals = [1.0; 10000];

    if let Ok(before) = statm_self() {
        for _ in 0..5000 {
            let _ = Image::new(10, 10, 2, &vals);
        }
        if let Ok(after) = statm_self() {
            assert_eq!(before, after);            
        }
    }
}
