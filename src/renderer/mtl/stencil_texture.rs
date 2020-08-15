use crate::Size;

pub fn create_stencil_texture_descriptor(size: Size) -> metal::TextureDescriptor {
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
}

impl StencilTexture {
    pub fn new(device: &metal::DeviceRef, size: Size) -> Self {
        let desc = create_stencil_texture_descriptor(size);
        let tex = device.new_texture(&desc);
        Self {
            device: device.to_owned(),
            tex,
            size,
        }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn tex(&self) -> &metal::TextureRef {
        &self.tex
    }

    pub fn resize(&mut self, size: Size) {
        if self.size.contains(&size) {
            return;
        }

        let desc = create_stencil_texture_descriptor(size);

        self.size = size;
        self.tex = self.device.new_texture(&desc);
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
