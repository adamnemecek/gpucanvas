use super::{StencilTexture, Vertex};
use metalgear::GPUVec;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct BufferIndex {
    inner: usize,
}

pub struct MtlBuffers {
    pub stencil_texture: StencilTexture,
    pub vertex_buffer: GPUVec<Vertex>,
    pub index_buffer: GPUVec<u32>,
}

impl MtlBuffers {
    pub fn new(device: &metal::DeviceRef, size: crate::Size) -> Self {
        Self {
            stencil_texture: StencilTexture::new(device, size),
            vertex_buffer: GPUVec::with_capacity(device, 32),
            index_buffer: GPUVec::with_capacity(device, 32),
        }
    }
}

struct MtlBufferCacheEntry {
    busy: bool,
    buffers: MtlBuffers,
}
pub struct MtlBuffersCache {
    device: metal::Device,
    inner: Vec<MtlBufferCacheEntry>,
}

impl MtlBuffersCache {
    pub fn new(device: &metal::DeviceRef, count: usize) -> Self {
        todo!()
        // Self {
        //  device: device.to_owned()
        // }
    }

    // finds an empty
    pub fn acquire(&mut self) -> (BufferIndex, MtlBuffers) {
        let fst = self.inner.iter().enumerate().find(|(i, x)| !x.busy);
        todo!()
    }

    pub fn release(&mut self, index: BufferIndex) {
        self.inner[index.inner].busy = false;
        todo!()
    }
}

unsafe impl Send for MtlBuffersCache {}
unsafe impl Sync for MtlBuffersCache {}
