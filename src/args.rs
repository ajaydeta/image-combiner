
#[derive(Debug)]
pub struct Args {
    pub image_1: String,
    pub image_2: String,
    pub out: String,
}

impl Args {
    pub fn new() -> Self {
        Args { 
            image_1: get_nth_arg(1),
            image_2: get_nth_arg(2), 
            out: get_nth_arg(3) 
        }
    }
}

fn get_nth_arg(n: usize) -> String {
    return std::env::args().nth(n).unwrap();
}