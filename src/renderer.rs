//! Module containing renderer implementations

use rgb::RGBA8;
use imgref::ImgVec;

use crate::{
    Color,
    Result,
    FillRule,
    Image,
    ImageId,
    ImageFlags,
    ImageStore,
    ImageSource,
    CompositeOperationState
};

mod opengl;
pub use opengl::OpenGl;

mod mtl;
pub use mtl::Mtl;

mod void;
pub use void::Void;

mod params;
pub(crate) use params::Params;

#[derive(Copy, Clone, Default)]
pub struct Drawable {
    pub(crate) fill_verts: Option<(usize, usize)>,
    pub(crate) stroke_verts: Option<(usize, usize)>,
}

#[derive(Debug)]
pub enum CommandType {
    ClearRect {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        color: Color
    },
    ConvexFill {
        params: Params
    },
    ConcaveFill {
        stencil_params: Params,
        fill_params: Params,
    },
    Stroke {
        params: Params
    },
    StencilStroke {
        params1: Params,
        params2: Params
    },
    Triangles {
        params: Params
    },
}

pub struct Command {
    pub(crate) cmd_type: CommandType,
    pub(crate) drawables: Vec<Drawable>,
    pub(crate) triangles_verts: Option<(usize, usize)>,
    pub(crate) image: Option<ImageId>,
    pub(crate) alpha_mask: Option<ImageId>,
    pub(crate) fill_rule: FillRule,
    pub(crate) composite_operation: CompositeOperationState
}

impl Command {
    pub fn new(flavor: CommandType) -> Self {
        Self {
            cmd_type: flavor,
            drawables: Default::default(),
            triangles_verts: Default::default(),
            image: Default::default(),
            alpha_mask: Default::default(),
            fill_rule: Default::default(),
            composite_operation: Default::default()
        }
    }
}

pub enum RenderTarget {
    Screen,
    Image(ImageId)
}

/// This is the main renderer trait that the [Canvas](../struct.Canvas.html) draws to.
pub trait Renderer {
    type Image: Image;

    fn set_size(&mut self, width: u32, height: u32, dpi: f32);

    fn render(&mut self, images: &ImageStore<Self::Image>, verts: &[Vertex], commands: &[Command]);

    fn create_image(&mut self, data: ImageSource, flags: ImageFlags) -> Result<Self::Image>;
    fn update_image(&mut self, image: &mut Self::Image, data: ImageSource, x: usize, y: usize) -> Result<()>;
    fn delete_image(&mut self, image: Self::Image);

    fn set_target(&mut self, images: &ImageStore<Self::Image>, target: RenderTarget);

    fn blur(&mut self, image: &mut Self::Image, amount: f32, x: usize, y: usize, width: usize, height: usize);

    fn screenshot(&mut self) -> Result<ImgVec<RGBA8>>;
}

/// Vertex struct for specifying triangle geometry
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
#[repr(C)]
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

#[derive(Copy, Clone)]
pub enum ShaderType {
    FillGradient,
    FillImage,
    Stencil,
}

impl Default for ShaderType {
    fn default() -> Self { Self::FillGradient }
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
