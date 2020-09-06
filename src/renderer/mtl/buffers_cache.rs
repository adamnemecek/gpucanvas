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
    semaphore: sema::Semaphore,
}

static mut FREED: Vec<BufferIndex> = Vec::new();

impl MtlBuffersCache {
    pub fn new(device: &metal::DeviceRef, count: usize) -> Self {
        let semaphore = sema::Semaphore::new(count as _);
        // Self {
        // devic
        // }
        todo!()
        // Self {
        //  device: device.to_owned()
        // }
    }

    // finds an empty
    pub fn acquire(&mut self, queue: &metal::CommandQueueRef) -> (BufferIndex, MtlBuffers, metal::CommandBuffer) {
        // select a buffer similarly as mtlnvg__renderViewport
        // wait on semaphore
        let _ = self.semaphore.wait();
        unsafe {
            for e in FREED.iter() {
                self.inner[e.inner].busy = false;
            }
            FREED.clear();
        }
        let command_buffer = queue.new_command_buffer();
        let (idx, buffers) = self.inner.iter().enumerate().find(|(i, x)| !x.busy).unwrap();
        let index = BufferIndex { inner: idx };
        // let ptr = &self.semaphore.as_pointer();
        let block = block::ConcreteBlock::new(move |buffer: &metal::CommandBufferRef| {
            //     // println!("{}", buffer.label());
            // self.vertex_buffer.clear();
            // self.release(index);
            unsafe {
                FREED.push(index);
            }

            // unlock();
        })
        .copy();
        command_buffer.add_completed_handler(&block);
        // (index, buffers.inner.to_owned())
        todo!()
    }

    pub fn release(&self, index: BufferIndex) {
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
