use image::DynamicImage;


pub fn combine_image(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    let vec_1 = image_1.to_bgra8().into_vec();
    let vec_2 = image_2.to_bgra8().into_vec();

    alternate_pixels(vec_1, vec_2)
}

pub fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    let mut combined_data = vec![0u8; vec_1.len()];

    let mut i = 0;
    while i < vec_1.len() {
        if i % 8 == 0 {
            combined_data.splice(i..= i+3, set_rgba(&vec_1, i, i+3));
        } else {
            combined_data.splice(i..= i+3, set_rgba(&vec_2, i, i+3));
        }
        i += 4;
    }

    combined_data
}

pub fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    
    for i in start..=end {
        let val = match vec.get(i) {
            Some(d) => *d,
            None => panic!("Index out of bounds"),
        };
        rgba.push(val);
    }

    rgba
}