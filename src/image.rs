
use rgb::*;
use rgb::alt::GRAY8;
use imgref::*;
use bitflags::bitflags;
use generational_arena::{Arena, Index};

#[cfg(feature = "image-loading")]
use ::image::DynamicImage;

#[cfg(feature = "image-loading")]
use std::convert::TryFrom;

use crate::{
    Result,
    ErrorKind,
    Renderer
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ImageId(pub Index);

// TODO: Rename those to RGB8, RGBA8, GRAY8 to better indicate size
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ImageFormat {
    Rgb,
    Rgba,
    Gray
}

bitflags! {
    pub struct ImageFlags: u32 {
        const GENERATE_MIPMAPS = 1;     // Generate mipmaps during creation of the image.
        const REPEAT_X = 1 << 1;        // Repeat image in X direction.
        const REPEAT_Y = 1 << 2;        // Repeat image in Y direction.
        const FLIP_Y = 1 << 3;          // Flips (inverses) image in Y direction when rendered.
        const PREMULTIPLIED = 1 << 4;   // Image data has premultiplied alpha.
        const NEAREST = 1 << 5;         // Image interpolation is Nearest instead Linear
    }
}

#[derive(Copy, Clone)]
#[non_exhaustive]
pub enum ImageSource<'a> {
    Rgb(ImgRef<'a, RGB8>),
    Rgba(ImgRef<'a, RGBA8>),
    Gray(ImgRef<'a, GRAY8>),
}

impl ImageSource<'_> {
    pub fn format(&self) -> ImageFormat {
        match self {
            Self::Rgb(_) => ImageFormat::Rgb,
            Self::Rgba(_) => ImageFormat::Rgb,
            Self::Gray(_) => ImageFormat::Gray,
        }
    }

    // TODO: Create size struct and use it here and in ImageInfo.
    pub fn dimensions(&self) -> (usize, usize) {
        match self {
            Self::Rgb(imgref) => (imgref.width(), imgref.height()),
            Self::Rgba(imgref) => (imgref.width(), imgref.height()),
            Self::Gray(imgref) => (imgref.width(), imgref.height()),
        }
    }
}

impl<'a> From<ImgRef<'a, RGB8>> for ImageSource<'a> {
    fn from(src: ImgRef<'a, RGB8>) -> Self {
        ImageSource::Rgb(src)
    }
}

impl<'a> From<ImgRef<'a, RGBA8>> for ImageSource<'a> {
    fn from(src: ImgRef<'a, RGBA8>) -> Self {
        ImageSource::Rgba(src)
    }
}

impl<'a> From<ImgRef<'a, GRAY8>> for ImageSource<'a> {
    fn from(src: ImgRef<'a, GRAY8>) -> Self {
        ImageSource::Gray(src)
    }
}

#[cfg(feature = "image-loading")]
impl<'a> TryFrom<&'a DynamicImage> for ImageSource<'a> {
    type Error = ErrorKind;

    fn try_from(src: &'a DynamicImage) -> Result<Self> {
        match src {
            ::image::DynamicImage::ImageLuma8(img) => {
                let src: Img<&[GRAY8]> = Img::new(
                    img.as_ref().as_pixels(),
                    img.width() as usize,
                    img.height() as usize
                );

                Ok(ImageSource::from(src))
            },
            ::image::DynamicImage::ImageRgb8(img) => {
                let src = Img::new(img.as_ref().as_rgb(), img.width() as usize, img.height() as usize);
                Ok(ImageSource::from(src))
            },
            ::image::DynamicImage::ImageRgba8(img) => {
                let src = Img::new(img.as_ref().as_rgba(), img.width() as usize, img.height() as usize);
                Ok(ImageSource::from(src))
            },
            // TODO: if format is not supported maybe we should convert it here,
            // Buut that is an expensive operation on the render thread that will remain hidden from the user
            _ => Err(ErrorKind::UnsuportedImageFromat)
        }
    }
}

#[derive(Copy, Clone)]
pub struct ImageInfo {
    flags: ImageFlags,
    width: usize,
    height: usize,
    format: ImageFormat
}

impl ImageInfo {
    pub fn new(flags: ImageFlags, width: usize, height: usize, format: ImageFormat) -> Self {
        Self { flags, width, height, format }
    }

    pub fn flags(&self) -> ImageFlags {
        self.flags
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn format(&self) -> ImageFormat {
        self.format
    }

    pub fn set_format(&mut self, format: ImageFormat) {
        self.format = format;
    }
}

pub trait Image {
    fn info(&self) -> ImageInfo;
}

pub struct ImageStore<T>(Arena<T>);

impl<T: Image> Default for ImageStore<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Image> ImageStore<T> {
    pub fn new() -> Self {
        Self(Arena::new())
    }

    pub fn add<R: Renderer<Image = T>>(&mut self, renderer: &mut R, data: ImageSource, flags: ImageFlags) -> Result<ImageId> {
        let image = renderer.create_image(data, flags)?;

        Ok(ImageId(self.0.insert(image)))
    }

    pub fn get(&self, id: ImageId) -> Option<&T> {
        self.0.get(id.0)
    }

    pub fn get_mut(&mut self, id: ImageId) -> Option<&mut T> {
        self.0.get_mut(id.0)
    }

    pub fn update<R: Renderer<Image = T>>(&mut self, renderer: &mut R, id: ImageId, data: ImageSource, x: usize, y: usize) -> Result<()> {
        if let Some(image) = self.0.get_mut(id.0) {
            renderer.update_image(image, data, x, y)?;
        } else {
            return Err(ErrorKind::ImageIdNotFound);
        }

        Ok(())
    }

    pub fn remove<R: Renderer<Image = T>>(&mut self, renderer: &mut R, id: ImageId) {
        if let Some(image) = self.0.remove(id.0) {
            renderer.delete_image(image);
        }
    }

    pub fn clear<R: Renderer<Image = T>>(&mut self, renderer: &mut R) {
        for (_idx, image) in self.0.drain() {
            renderer.delete_image(image);
        }
    }
}
