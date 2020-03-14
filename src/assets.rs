use crate::data::load_image;
use crate::gl::{Gl, Image};
use anyhow::Result;
use std::collections::HashMap;
use std::rc::Rc;
use vek::Vec2;

pub struct Assets {
    gl: Rc<Gl>,
    path_to_index: HashMap<&'static str, ImageId>,
    images: Vec<Image>,
}

impl Assets {
    pub fn new(gl: Rc<Gl>) -> Self {
        Assets {
            gl,
            path_to_index: HashMap::new(),
            images: Vec::new(),
        }
    }

    pub async fn load(&mut self, path: &'static str) -> Result<ImageId> {
        let element = load_image(path).await?;
        let image = Image::from_image_element(self.gl.clone(), &element);

        let id = ImageId {
            id: self.images.len() as u32,
        };
        self.path_to_index.insert(path, id);
        self.images.push(image);
        Ok(id)
    }

    pub fn find_id(&self, path: &str) -> Option<ImageId> {
        self.path_to_index.get(path).map(|id| *id)
    }

    pub fn image(&self, image_id: ImageId) -> Option<&Image> {
        self.images.get(image_id.id as usize)
    }

    pub fn images(&self) -> Vec<ImageId> {
        self.images
            .iter()
            .enumerate()
            .map(|(i, _)| ImageId { id: i as u32 })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageId {
    pub id: u32,
}

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
        let id = ImageId {
            id: self.images.len() as u32,
        };

        let image = Image::from_image_element(self.gl.clone(), &element);

        self.images.push(image);
        self.names_to_id.insert(path.to_string(), id);
        Ok(id)
    }

    pub fn size(&self, image_id: ImageId) -> Vec2<u16> {
        self.images
            .get(image_id.id as usize)
            .map(|image| Vec2::new(image.width, image.height))
            .unwrap_or_default()
    }

    pub fn image(&self, image_id: ImageId) -> Option<&Image> {
        self.images.get(image_id.id as usize)
    }

    pub fn find_image_id(&self, path: &str) -> Option<ImageId> {
        self.names_to_id.get(path).map(|id| *id)
    }

    pub fn images(&self) -> Vec<(ImageId, &Image)> {
        self.images
            .iter()
            .enumerate()
            .map(|(i, image)| (ImageId { id: i as u32 }, image))
            .collect()
    }
}
