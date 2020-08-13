#![allow(unused_variables)]

use imgref::ImgVec;
use rgb::RGBA8;

use crate::{ErrorKind, ImageId, ImageInfo, ImageSource, ImageStore};

use super::{Command, Renderer, Vertex};

/// Void renderer used for testing
pub struct Void;

impl Renderer for Void {
    type Image = VoidImage;

    fn set_size(&mut self, width: u32, height: u32, dpi: f32) {}

    fn render(&mut self, images: &ImageStore<VoidImage>, verts: &[Vertex], commands: &[Command]) {}

    fn alloc_image(&mut self, info: ImageInfo) -> Result<Self::Image, ErrorKind> {
        Ok(VoidImage { info })
    }

    fn start_capture(&self) {}

    fn stop_capture(&self) {}

    fn label(&self, images: &ImageStore<Self::Image>, id: ImageId) -> String {
        "labels not supported for void backend".to_owned()
    }

    fn set_label(&self, images: &ImageStore<Self::Image>, id: ImageId, label: &str) {}

    fn update_image(
        &mut self,
        image: &mut Self::Image,
        data: ImageSource,
        x: usize,
        y: usize,
    ) -> Result<(), ErrorKind> {
        let size = data.dimensions();

        if x + size.0 > image.info.width() {
            return Err(ErrorKind::ImageUpdateOutOfBounds);
        }

        if y + size.1 > image.info.height() {
            return Err(ErrorKind::ImageUpdateOutOfBounds);
        }

        Ok(())
    }

    fn delete_image(&mut self, image: Self::Image) {}

    fn screenshot(&mut self, _images: &ImageStore<Self::Image>) -> Result<ImgVec<RGBA8>, ErrorKind> {
        Ok(ImgVec::new(Vec::new(), 0, 0))
    }
}

pub struct VoidImage {
    info: ImageInfo,
}
