//! Module containing renderer implementations

use imgref::ImgVec;
use rgb::RGBA8;

use crate::{Color, CompositeOperationState, ErrorKind, FillRule, ImageId, ImageInfo, ImageSource, ImageStore, Size};

mod opengl;
pub use opengl::OpenGl;

mod mtl;
pub use mtl::{Mtl, MtlStencilTexture, MtlTexture};

mod void;
pub use void::Void;

mod params;
pub(crate) use params::Params;

#[derive(Copy, Clone, Default)]
pub struct Drawable {
    pub(crate) fill_verts: Option<(usize, usize)>,
    pub(crate) stroke_verts: Option<(usize, usize)>,
    // pub(crate) index_verts: Option<(usize, usize)>,
}

// pub type GPUC = fn() -> ();
pub trait CommandEncoder: std::fmt::Debug {
    fn encode(&self, encoder: &metal::RenderCommandEncoderRef);
}

#[derive(Debug)]
pub enum CommandType {
    CustomCommand {
        command_encoder: std::sync::Arc<dyn CommandEncoder>,
    },
    SetRenderTarget(RenderTarget),
    GPUTriangle,
    ClearRect {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        color: Color,
    },
    ConvexFill {
        params: Params,
    },
    ConcaveFill {
        stencil_params: Params,
        fill_params: Params,
    },
    Stroke {
        params: Params,
    },
    StencilStroke {
        params1: Params,
        params2: Params,
    },
    Triangles {
        params: Params,
    },
    Blit {
        source: ImageId,
        destination_origin: (u32, u32),
    },
}

pub struct Command {
    pub(crate) cmd_type: CommandType,
    pub(crate) drawables: Vec<Drawable>,
    pub(crate) triangles_verts: Option<(usize, usize)>,
    pub(crate) image: Option<ImageId>,
    pub(crate) alpha_mask: Option<ImageId>,
    pub(crate) fill_rule: FillRule,
    pub(crate) composite_operation: CompositeOperationState,
}

impl Command {
    pub fn new(cmd_type: CommandType) -> Self {
        Self {
            cmd_type,
            drawables: Default::default(),
            triangles_verts: Default::default(),
            image: Default::default(),
            alpha_mask: Default::default(),
            fill_rule: Default::default(),
            composite_operation: Default::default(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RenderTarget {
    None,
    Screen,
    Image(ImageId),
}

pub trait BufferCache {}

pub struct VoidCache {
    _inner: u32,
}

impl VoidCache {
    pub fn new() -> Self {
        Self { _inner: 0 }
    }
}

impl BufferCache for VoidCache {}

/// This is the main renderer trait that the [Canvas](../struct.Canvas.html) draws to.
pub trait Renderer {
    type Image;
    ///
    /// Use the cache for [Multiple buffering](https://en.wikipedia.org/wiki/Multiple_buffering)
    ///
    type BufferCache: BufferCache;

    fn set_size(&mut self, width: u32, height: u32, dpi: f32);

    fn alloc_buffer_cache(&self) -> Self::BufferCache;

    fn render(
        &mut self,
        images: &ImageStore<Self::Image>,
        cache: &mut Self::BufferCache,
        verts: &[Vertex],
        commands: &[Command],
    );

    fn alloc_image(&mut self, info: ImageInfo) -> Result<Self::Image, ErrorKind>;
    fn update_image(&mut self, image: &mut Self::Image, data: ImageSource, x: usize, y: usize)
        -> Result<(), ErrorKind>;
    fn delete_image(&mut self, image: Self::Image);

    fn screenshot(&mut self, images: &ImageStore<Self::Image>) -> Result<ImgVec<RGBA8>, ErrorKind>;

    fn flip_y() -> bool;
    fn flip_uv() -> bool;

    fn start_capture(&self);
    fn stop_capture(&self);

    fn label(&self, images: &ImageStore<Self::Image>, id: ImageId) -> String;
    fn set_label(&self, images: &ImageStore<Self::Image>, id: ImageId, label: &str);

    fn view_size(&self) -> Size;
}

/// Vertex struct for specifying triangle geometry
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub u: f32,
    pub v: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, u: f32, v: f32) -> Self {
        Self { x, y, u, v }
    }

    pub fn set(&mut self, x: f32, y: f32, u: f32, v: f32) {
        *self = Self { x, y, u, v };
    }
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum ShaderType {
    FillGradient,
    FillImage,
    Stencil,
}

impl Default for ShaderType {
    fn default() -> Self {
        Self::FillGradient
    }
}

impl ShaderType {
    pub fn to_f32(self) -> f32 {
        match self {
            Self::FillGradient => 0.0,
            Self::FillImage => 1.0,
            Self::Stencil => 2.0,
        }
    }
}
