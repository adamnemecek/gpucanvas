use super::{StencilTexture, Vertex};
use metalgear::GPUVec;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct BufferIndex {
    inner: usize,
}

pub struct Buffers {
    pub stencil_texture: StencilTexture,
    pub index_buffer: GPUVec<u32>,
    pub vertex_buffer: GPUVec<Vertex>,
}

impl Buffers {
    pub fn new(device: &metal::DeviceRef, size: crate::Size) -> Self {
        // Self {
        //     stencil_texture: StencilTexture::new(device, size),

        // }
        todo!()
    }
}

struct BufferCacheEntry {
    busy: bool,
    buffers: Buffers,
}
pub struct BuffersCache {
    device: metal::Device,
    inner: Vec<BufferCacheEntry>,
}

impl BuffersCache {
    pub fn new(device: &metal::DeviceRef, count: usize) -> Self {
        todo!()
        // Self {
        //  device: device.to_owned()
        // }
    }

    // finds an empty
    pub fn acquire(&mut self) -> (BufferIndex, Buffers) {
        let fst = self.inner.iter().enumerate().find(|(i, x)| !x.busy);
        todo!()
    }

    pub fn release(&mut self, index: BufferIndex) {
        self.inner[index.inner].busy = false;
        todo!()
    }
}

unsafe impl Send for BuffersCache {}
unsafe impl Sync for BuffersCache {}
