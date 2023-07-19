
use crate::errors::ImageDataErrors;

pub struct FloatingImage {
    pub width: u32,
    pub height: u32,
    pub name: String,
    pub data: Vec<u8>,
}

impl FloatingImage {
    pub fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = 3_655_744;
        let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
        FloatingImage { 
            width,
            height,
            data: buffer,
            name 
        }
    }

    pub fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }

        self.data = data;

        Ok(())
    }

}
