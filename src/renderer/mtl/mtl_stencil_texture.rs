use super::MtlTextureExt;
use crate::Size;

pub struct MtlStencilTexture {
    device: metal::Device,
    tex: metal::Texture,
    // size: Size,
    gen: u32,
}

impl MtlStencilTexture {
    pub fn new(device: &metal::DeviceRef, size: Size) -> Self {
        let desc = metal::TextureDescriptor::new_stencil_descriptor(size);
        let tex = device.new_texture(&desc);
        let gen = 0;

        tex.set_label(&format!("stencil texture gen: {:?}", gen));
        Self {
            device: device.to_owned(),
            tex,
            // size,
            gen,
        }
    }

    pub fn device(&self) -> &metal::DeviceRef {
        &self.device
    }

    pub fn set_device(&mut self, device: &metal::DeviceRef) {
        let size = self.size();
        self.device = device.to_owned();
        self.resize(size);
    }

    pub fn tex(&self) -> &metal::TextureRef {
        &self.tex
    }

    #[inline]
    pub fn size(&self) -> Size {
        self.tex.size()
    }

    pub fn resize(&mut self, size: Size) {
        // todo fix adam
        if self.size().contains(&size) {
            return;
        }
        println!("resizing stencil from {:?} to {:?}", self.size(), size);

        // use `max` as opposed to the size because let's say we want to stencil two rectangles
        // one vertical, one horizontal. if we just use the new size, we will be allocating
        // and releasing a lot. the max accomodates both of them.
        let size = size.max(&self.size());
        let desc = metal::TextureDescriptor::new_stencil_descriptor(size);

        // self.size = size;
        self.gen += 1;

        self.tex.set_purgeable_state(metal::MTLPurgeableState::Empty);
        self.tex = self.device.new_texture(&desc);

        self.tex.set_label(&format!("stencil texture gen: {:?}", self.gen));
    }

    // pub fn label(&self) -> &str {
    //     self.tex.label()
    // }

    // pub fn set_label(&self, label: &str) {
    //     self.tex.set_label(label)
    // }

    // pub fn clear(&mut self) {
    //     self.size = Size::default();
    // }

    pub fn to_owned(&self) -> Self {
        Self {
            tex: self.tex.to_owned(),
            device: self.device.to_owned(),
            // size: self.size,
            gen: self.gen,
        }
    }
}

trait TextureDescriptorExt {
    fn new_stencil_descriptor(size: Size) -> Self;
}

impl TextureDescriptorExt for metal::TextureDescriptor {
    fn new_stencil_descriptor(size: Size) -> Self {
        let desc = metal::TextureDescriptor::new();
        desc.set_texture_type(metal::MTLTextureType::D2);
        desc.set_pixel_format(metal::MTLPixelFormat::Stencil8);

        desc.set_width(size.w as u64);
        desc.set_height(size.h as u64);
        desc.set_mipmap_level_count(1);

        #[cfg(target_os = "macos")]
        {
            desc.set_resource_options(metal::MTLResourceOptions::StorageModePrivate);
        }
        desc.set_usage(metal::MTLTextureUsage::RenderTarget);
        desc
    }
}
