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

    fn release(&mut self) {
        self.vertex_buffer.clear();
        self.index_buffer.clear();
    }

    fn is_ident(&self, other: &Self) -> bool {
        // self.stencil_texture.
        self.vertex_buffer.ptr_hash() == other.vertex_buffer.ptr_hash()
    }

    pub fn to_owned(&self) -> Self {
        let ret = Self {
            stencil_texture: self.stencil_texture.to_owned(),
            vertex_buffer: self.vertex_buffer.to_owned(),
            index_buffer: self.index_buffer.to_owned(),
        };
        assert!(self.is_ident(&ret));
        ret
    }
}

struct MtlBufferCacheEntry {
    inner: MtlBuffers,
    busy: bool,
    // command_buffer: Option<metal::CommandBuffer>,
}
pub struct MtlBuffersCache {
    device: metal::Device,
    inner: Vec<MtlBufferCacheEntry>,
    // semaphore
}

impl MtlBuffersCache {
    pub fn new(device: &metal::DeviceRef, count: usize) -> Self {
        todo!()
        // Self {
        //  device: device.to_owned()
        // }
    }

    // finds an empty
    pub fn acquire(&mut self, queue: &metal::CommandQueueRef) -> (BufferIndex, MtlBuffers) {
        // select a buffer similarly as mtlnvg__renderViewport
        // wait on semaphore
        let (idx, buffers) = self.inner.iter().enumerate().find(|(i, x)| !x.busy).unwrap();
        (BufferIndex { inner: idx }, buffers.inner.to_owned())
    }

    pub fn release(&mut self, index: BufferIndex) {
        // call buffers clear
        // set busy to false
        // signal_semaphore
        // let x = &mutself.inner[index.inner];
        // self.inner[index.inner].busy = false;
        todo!()
    }
}

unsafe impl Send for MtlBuffersCache {}
unsafe impl Sync for MtlBuffersCache {}
