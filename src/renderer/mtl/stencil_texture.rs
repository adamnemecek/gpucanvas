use crate::Size;

fn create_stencil_texture_descriptor(size: Size) -> metal::TextureDescriptor {
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

pub struct StencilTexture {
    device: metal::Device,
    tex: metal::Texture,
    size: Size,
    gen: u32,
}

impl StencilTexture {
    pub fn new(device: &metal::DeviceRef, size: Size) -> Self {
        let desc = create_stencil_texture_descriptor(size);
        let tex = device.new_texture(&desc);
        let gen = 0;

        tex.set_label(&format!("gen: {:?}", gen));
        Self {
            device: device.to_owned(),
            tex,
            size,
            gen,
        }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn tex(&self) -> &metal::TextureRef {
        &self.tex
    }

    pub fn resize(&mut self, size: Size) {
        // todo fix adam
        if self.size.contains(&size) {
            return;
        }
        println!("resizing stencil from {:?} to {:?}", self.size, size);

        // use a max as opposed to the size because let's say we want to stencil two rectangles
        // one vertical, one horizontal. if we just use the new size, we will be allocating
        // and releasing a lot. the max accomodates both of them.
        let size = size.max(&self.size);
        let desc = create_stencil_texture_descriptor(size);

        self.size = size;
        self.gen += 1;

        self.tex.set_purgeable_state(metal::MTLPurgeableState::Empty);
        self.tex = self.device.new_texture(&desc);

        self.tex.set_label(&format!("stencil texture gen: {:?}", self.gen));
    }

    pub fn label(&self) -> &str {
        self.tex.label()
    }

    pub fn set_label(&self, label: &str) {
        self.tex.set_label(label)
    }
    // pub fn clear(&mut self) {
    //     self.size = Size::default();
    // }
}
