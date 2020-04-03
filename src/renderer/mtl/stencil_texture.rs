

pub fn create_stencil_texture_descriptor(size: (f32, f32)) -> metal::TextureDescriptor {
    let desc = metal::TextureDescriptor::new();
    desc.set_texture_type(metal::MTLTextureType::D2);
    desc.set_pixel_format(metal::MTLPixelFormat::Stencil8);
    desc.set_resource_options(metal::MTLResourceOptions::CPUCacheModeDefaultCache);

    desc.set_width(size.0 as u64);
    desc.set_height(size.0 as u64);
    desc.set_mipmap_level_count(1);

    desc.set_resource_options(metal::MTLResourceOptions::StorageModePrivate);
    desc.set_usage(metal::MTLTextureUsage::RenderTarget);
    desc
}

pub struct StencilTexture {
    pub device: metal::Device,
    pub tex: metal::Texture,
    pub size: (f32, f32),
}

impl StencilTexture {
    pub fn new(device: &metal::DeviceRef, size: (f32, f32)) -> Self {
        let desc = create_stencil_texture_descriptor(size);
        let tex = device.new_texture(&desc);
        Self {
            device: device.to_owned(),
            tex,
            size,
        }
    }

    // pub fn width(&self) -> usize {
    //     // if let Some(v) = self.tex.map(|x| x.width()) {
    //     //     v
    //     // }
    //     // else {
    //     //     0
    //     // }
    // }

    // pub fn height(&self) -> usize {
    //     if let Some(v) = self.tex.map(|x| x.height()) {
    //         v
    //     }
    //     0
    // }

    pub fn resize(&mut self, size: (f32, f32)) {
        // if self.size.contains(&size) {
        //     return;
        // }

        let desc = create_stencil_texture_descriptor(size);

        self.size = size;
        self.tex = self.device.new_texture(&desc);
    }

    pub fn clear(&mut self) {
        // self.size = ViewSize::zero();
    }

    // pub fn into_(&self) -> &'_ metal::TextureRef {
    //     todo!()
    // }


}

// impl<'a> AsRef<metal::Texture> for &'a StencilTexture {
//     fn as_ref(self) -> &'a metal::TextureRef {
//         todo!()
//     }
// }




// impl<'a> Into<&'a metal::TextureRef> for StencilTexture<'a> {
//     fn into(self) -> &'a metal::TextureRef {
//         // self.tex.as_ref()
//         todo!()
//     }
// }