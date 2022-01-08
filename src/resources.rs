use crate::prelude::*;
use std::collections::HashMap;

pub const PUZZLES: usize = 12;
pub struct TResources {
    pub files: HashMap<usize, String>,
    pub images: Vec<graphics::Image>,
}
impl TResources {
    pub fn new_resources(ctx: &mut Context) -> Self {
        let mut files = HashMap::new();

        files.insert(0, "Amsterdam.png".to_string());
        files.insert(1, "Athens.png".to_string());
        files.insert(2, "Berlin.png".to_string());
        files.insert(3, "Bern.png".to_string());
        files.insert(4, "Brussels.png".to_string());
        files.insert(5, "Lisbon.png".to_string());
        files.insert(6, "London.png".to_string());
        files.insert(7, "Madrid.png".to_string());
        files.insert(8, "Paris.png".to_string());
        files.insert(9, "Prague.png".to_string());
        files.insert(10, "Rome.png".to_string());
        files.insert(11, "Vienna.png".to_string());

        let mut images = Vec::new();

        for index in 0..PUZZLES {
            let filename = &files[&index];
            let image = graphics::Image::new(ctx, filename).unwrap();
            images.push(image);
        }
        Self { files, images }
    }
    pub fn get_image(&mut self, index: usize) -> graphics::Image {
        self.images[index].clone()
    }
}
