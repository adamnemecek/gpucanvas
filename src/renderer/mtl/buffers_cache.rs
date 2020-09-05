use super::{StencilTexture, Vertex};
use metalgear::GPUVec;

pub struct BufferIndex {
    inner: usize
}


pub struct Buffers {
    pub stencil_texture: StencilTexture,
    pub index_buffer: GPUVec<u32>,
    pub vertex_buffer: GPUVec<Vertex>,
}
pub struct BuffersCache {
    inner: Vec<Buffers>,
}

impl BuffersCache {
    pub fn new(count: usize) -> Self {
        todo!()
    }

    pub fn finished(&mut self, index: BufferIndex) {
        todo!()
    }
}


unsafe impl Send for BuffersCache { }
unsafe impl Sync for BuffersCache { }