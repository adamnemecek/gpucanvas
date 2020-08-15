use super::Blend;
use std::hash::{Hash, Hasher};

pub struct RPS {
    pub device: metal::Device,
    pub blend_func: Blend,
    pub pipeline_state: metal::RenderPipelineState,
    pub stencil_only_pipeline_state: metal::RenderPipelineState,
    pub pipeline_pixel_format: metal::MTLPixelFormat,

    pub clear_rect_pipeline_state: Option<metal::RenderPipelineState>,
}

impl Hash for RPS {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}

pub struct RPSCache {
    pub device: metal::Device,
}

impl RPSCache {}
