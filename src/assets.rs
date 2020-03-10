use crate::data::load_image;
use crate::gl::{Gl, Image};
use anyhow::Result;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Assets {
    gl: Rc<Gl>,
    images: Vec<Image>,
}

impl Assets {
    pub fn new(gl: Rc<Gl>) -> Self {
        Assets {
            gl,
            images: Vec::new(),
        }
    }

    pub async fn load(&mut self, path: &str) -> Result<ImageId> {
        let element = load_image(path).await?;
        let image = Image::from_image_element(self.gl.clone(), &element);

        let id = ImageId(self.images.len() as u32);
        self.images.push(image);
        Ok(id)
    }

    pub fn image(&self, image_id: ImageId) -> Option<&Image> {
        self.images.get(image_id.0 as usize)
    }

    pub fn images(&self) -> Vec<ImageId> {
        self.images
            .iter()
            .enumerate()
            .map(|(i, _)| ImageId(i as u32))
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageId(pub u32);

pub struct Images {
    gl: Rc<Gl>,
    names_to_id: HashMap<String, ImageId>,
    images: Vec<Image>,
}

impl Images {
    pub fn new(gl: Rc<Gl>) -> Self {
        Images {
            gl,
            names_to_id: HashMap::new(),
            images: Vec::new(),
        }
    }

    pub async fn load(&mut self, path: &str) -> Result<ImageId> {
        let element = load_image(path).await?;
        let image = Image::from_image_element(self.gl.clone(), &element);

        let id = ImageId(self.images.len() as u32);
        self.images.push(image);
        self.names_to_id.insert(path.to_string(), id);
        Ok(id)
    }

    pub fn image(&self, image_id: ImageId) -> Option<&Image> {
        self.images.get(image_id.0 as usize)
    }

    pub fn find_image_id(&self, path: &str) -> Option<ImageId> {
        self.names_to_id.get(path).map(|id| *id)
    }

    pub fn images(&self) -> Vec<(ImageId, &Image)> {
        self.images
            .iter()
            .enumerate()
            .map(|(i, image)| (ImageId(i as u32), image))
            .collect()
    }
}
