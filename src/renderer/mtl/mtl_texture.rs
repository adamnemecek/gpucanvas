use super::MtlTextureExt;
use crate::{ErrorKind, ImageFlags, ImageInfo, ImageSource, PixelFormat, Size};
use rgb::ComponentBytes;

impl From<PixelFormat> for metal::MTLPixelFormat {
    fn from(a: PixelFormat) -> Self {
        match a {
            // PixelFormat::Rgba8 | PixelFormat::Rgb8
            //  => metal::MTLPixelFormat::RGBA8Unorm,
            PixelFormat::Rgba8 => Self::BGRA8Unorm,
            PixelFormat::Rgb8 => unimplemented!("metal doesn't support the RGB8 pixel format"),
            PixelFormat::Gray8 => Self::R8Unorm,
        }
    }
}

pub struct MtlTexture {
    pub info: ImageInfo,
    tex: metal::Texture,
    sampler: metal::SamplerState,
    // todo: texture has a device reference, use that
    pub device: metal::Device,
    pub command_queue: metal::CommandQueue,
}

impl MtlTexture {
    pub fn new_pseudo_texture(
        device: &metal::DeviceRef,
        command_queue: &metal::CommandQueueRef,
    ) -> Result<Self, ErrorKind> {
        let info = ImageInfo::new(ImageFlags::empty(), 1, 1, PixelFormat::Gray8);
        Self::new(device, command_queue, info)
    }

    // called renderCreateTextureWithType...
    pub fn new(
        device: &metal::DeviceRef,
        command_queue: &metal::CommandQueueRef,
        info: ImageInfo,
    ) -> Result<Self, ErrorKind> {
        assert!(info.format() != PixelFormat::Rgb8);

        if info.format() == PixelFormat::Gray8 {
            println!("creating grey texture of size: {}, {}", info.width(), info.height());
        }

        let generate_mipmaps = info.flags().contains(ImageFlags::GENERATE_MIPMAPS);
        let nearest = info.flags().contains(ImageFlags::NEAREST);
        let repeatx = info.flags().contains(ImageFlags::REPEAT_X);
        let repeaty = info.flags().contains(ImageFlags::REPEAT_Y);

        let pixel_format = info.format().into();

        let desc = metal::TextureDescriptor::new();
        desc.set_texture_type(metal::MTLTextureType::D2);
        desc.set_pixel_format(pixel_format);

        desc.set_width(info.width() as u64);
        desc.set_height(info.height() as u64);

        if generate_mipmaps {
            let size = metal::MTLSize {
                width: info.width() as u64,
                height: info.height() as u64,
                depth: 1,
            };
            desc.set_mipmap_level_count_for_size(size);
        } else {
            desc.set_mipmap_level_count(1);
        };

        desc.set_usage(
            metal::MTLTextureUsage::RenderTarget
                | metal::MTLTextureUsage::ShaderWrite
                | metal::MTLTextureUsage::ShaderRead,
        );

        // todo if simulator
        // desc.set_resource_options(metal::MTLResourceOptions::StorageModePrivate);

        let tex = device.new_texture(&desc);

        if generate_mipmaps {
            tex.generate_mipmaps(command_queue);
        }

        let sampler_desc = metal::SamplerDescriptor::new();
        let mut label = String::new();

        if nearest {
            sampler_desc.set_min_filter(metal::MTLSamplerMinMagFilter::Nearest);
            sampler_desc.set_mag_filter(metal::MTLSamplerMinMagFilter::Nearest);
            if generate_mipmaps {
                sampler_desc.set_mip_filter(metal::MTLSamplerMipFilter::Nearest);
            }
            label.push_str("nearest, ");
            // println!("nearest");
        } else {
            sampler_desc.set_min_filter(metal::MTLSamplerMinMagFilter::Linear);
            sampler_desc.set_mag_filter(metal::MTLSamplerMinMagFilter::Linear);
            if generate_mipmaps {
                sampler_desc.set_mip_filter(metal::MTLSamplerMipFilter::Linear);
            }
            label.push_str("linear, ");
            // println!("linear");
        }

        if repeatx {
            sampler_desc.set_address_mode_s(metal::MTLSamplerAddressMode::Repeat);
            label.push_str("repeat_x, ");
        } else {
            sampler_desc.set_address_mode_s(metal::MTLSamplerAddressMode::ClampToEdge);
            label.push_str("clamp_x, ");
        }

        if repeaty {
            sampler_desc.set_address_mode_t(metal::MTLSamplerAddressMode::Repeat);
            label.push_str("repeat_y, ");
        } else {
            sampler_desc.set_address_mode_t(metal::MTLSamplerAddressMode::ClampToEdge);
            label.push_str("clamp_y, ");
        }

        sampler_desc.set_label(&label);

        let sampler = device.new_sampler(&sampler_desc);

        Ok(Self {
            device: device.to_owned(),
            command_queue: command_queue.to_owned(),
            info,
            tex,
            sampler,
        })
    }

    pub fn tex(&self) -> &metal::TextureRef {
        &self.tex
    }

    pub fn sampler(&self) -> &metal::SamplerStateRef {
        &self.sampler
    }

    #[inline]
    pub fn replace_region(&self, region: metal::MTLRegion, data: &[u8], stride: usize) {
        self.tex
            .replace_region(region, 0, data.as_ptr() as *const _, stride as u64)
    }

    pub fn set_clear_color(&self, color: rgb::RGBA8) {
        let w = self.tex().width();
        let h = self.tex().height();
        let region = metal::MTLRegion::new_2d(0, 0, w, h);

        let bytes = vec![color; (w * h) as _];
        self.replace_region(region, bytes.as_bytes(), (4 * w) as _);
    }

    pub fn size(&self) -> Size {
        // todo adam
        let a = Size::new(self.info.width() as _, self.info.height() as _);
        let b = self.tex().size();
        assert!(a == b);
        a
    }

    pub fn resize(&mut self, size: Size) {
        // todo fix adam
        if self.size().contains(&size) {
            return;
        }

        let mut info = self.info;
        info.set_size(size);
        // should i copy previous things over?

        *self = Self::new(&self.device, &self.command_queue, info).unwrap();
    }

    pub fn update(&mut self, src: ImageSource, x: usize, y: usize) -> Result<(), ErrorKind> {
        assert!(src.format() != PixelFormat::Rgb8);

        let (width, height) = src.dimensions();
        if x + width > self.info.width() {
            return Err(ErrorKind::ImageUpdateOutOfBounds);
        }

        if y + height > self.info.height() {
            return Err(ErrorKind::ImageUpdateOutOfBounds);
        }

        if self.info.format() != src.format() {
            return Err(ErrorKind::ImageUpdateWithDifferentFormat);
        }

        let region = metal::MTLRegion::new_2d(x as _, y as _, width as _, height as _);
        let stride: usize;
        // let data_offset: usize;
        let data;

        match src {
            ImageSource::Gray(data_) => {
                stride = width;
                // data_offset = y * stride + x;
                data = data_.buf().as_bytes();
            }
            ImageSource::Rgba(data_) => {
                stride = 4 * width;
                // data_offset = y * stride + x * 4;
                data = data_.buf().as_bytes();
            }
            ImageSource::Rgb(_) => {
                unimplemented!(
                    "Metal backend doesn't support RGB pixel format. Image should have been converted in load_image_file"
                );
            }
        }

        self.replace_region(region, data, stride);

        let generate_mipmaps = self.info.flags().contains(ImageFlags::GENERATE_MIPMAPS);
        if generate_mipmaps {
            self.tex.generate_mipmaps(&self.command_queue);
        }

        Ok(())
    }

    pub fn delete(self) {
        self.tex.set_purgeable_state(metal::MTLPurgeableState::Empty);
        // unsafe {
        //     gl::DeleteTextures(1, &self.id);
        // }
    }

    pub fn save_to(&self, path: &str) {
        // use super::MtlTextureExt;
        self.tex.save_to(path)
    }

    pub fn info(&self) -> ImageInfo {
        self.info
    }

    pub fn label(&self) -> &str {
        self.tex.label()
    }

    pub fn set_label(&self, label: &str) {
        self.tex.set_label(label);
    }
}

// this is from nvg-metal
// pub struct Texture {
//     pub id: crate::ImageId,
//     pub tex: Option<metal::Texture>,
//     pub sampler: Option<metal::SamplerState>,
//     pub texture_type: nvg::renderer::TextureType,
//     pub flags: nvg::renderer::ImageFlags,
// }

// impl Texture {
//     pub fn create_texture(
//         id: crate::ImageId,
//         texture_type: nvg::renderer::TextureType,
//         width: usize,
//         height: usize,
//         flags: nvg::renderer::ImageFlags,
//         data: Option<&[u8]>,
//     ) -> Self {

//         let pixel_format = match texture_type {
//             nvg::renderer::TextureType::Alpha => metal::MTLPixelFormat::R8Unorm,
//             nvg::renderer::TextureType::RGBA => metal::MTLPixelFormat::RGBA8Unorm,
//         };

//         // let mut tex = self.alloc_texture();
//         // tex.texture_type = texture_type;
//         // tex.flags = flags;

//         let tex_desc = {
//             let desc = metal::TextureDescriptor::new();
//             desc.set_texture_type(metal::MTLTextureType::D2);
//             desc.set_width(width as u64);
//             desc.set_height(height as u64);
//             desc.set_mipmap_level_count(1);
//             let usage = metal::MTLTextureUsage::ShaderRead | metal::MTLTextureUsage::RenderTarget | metal::MTLTextureUsage::ShaderWrite;
//             desc.set_usage(usage);
//             desc
//         };

//         // tex.tex = self.device.new_texture(&tex_desc);

//         if let Some(data) = data {
//             let stride = match texture_type {
//                 nvg::renderer::TextureType::Alpha => width,
//                 nvg::renderer::TextureType::RGBA => width * 4,
//             };

//             if tex_desc.storage_mode() == metal::MTLStorageMode::Private {

//             }
//             else {
//                 let region = metal::MTLRegion {
//                     origin: metal::MTLOrigin { x: 0, y: 0, z: 0 },
//                     size: metal::MTLSize { width: width as u64, height: height as u64, depth: 0 },
//                 };
//                 // tex.replace_region(region, 0, stride as u64, data.as_ptr() as *const _);
//             }
//         }
//         else {

//         }

//         let mut sampler_desc = metal::SamplerDescriptor::new();
//         let filter = if flags.contains(nvg::ImageFlags::NEAREST) {
//             metal::MTLSamplerMinMagFilter::Nearest
//         }
//         else {
//             metal::MTLSamplerMinMagFilter::Linear
//          };
//         sampler_desc.set_min_filter(filter);
//         sampler_desc.set_mag_filter(filter);

//         if flags.contains(nvg::ImageFlags::GENERATE_MIPMAPS) {

//         }

//         let address_mode_s =
//             if flags.contains(nvg::ImageFlags::REPEATX) {
//                 metal::MTLSamplerAddressMode::Repeat
//             }
//             else {
//                 metal::MTLSamplerAddressMode::ClampToEdge
//             };
//         sampler_desc.set_address_mode_s(address_mode_s);

//         let address_mode_t =
//             if flags.contains(nvg::ImageFlags::REPEATX) {
//                 metal::MTLSamplerAddressMode::Repeat
//             }
//             else {
//                 metal::MTLSamplerAddressMode::ClampToEdge
//             };
//         sampler_desc.set_address_mode_t(address_mode_t);
//         todo!()

//         // Self {

//         // }
//     }
//     pub fn create(id: crate::ImageId) -> Self {
//         todo!()
//     }

//     pub fn width(&self) -> usize {
//         // self.tex.map(|t|t.width()).or_else(0)
//         todo!()
//     }

//     pub fn height(&self) -> usize {
//         // self.tex.map(|t|t.height()).or_else(0)
//         todo!()
//     }

//     pub fn size(&self) -> (usize, usize) {
//         (self.width(), self.height())
//     }

//     pub fn replace_region(
//         &self,
//         region: metal::MTLRegion,
//         mipmap_level: usize,
//         stride: usize,
//         data: &[u8]
//     ) {
//         self.tex.as_ref().unwrap().replace_region(
//             region,
//             mipmap_level as u64,
//             stride as u64,
//             data.as_ptr() as *const _
//         )
//     }

//     pub fn update(
//         &self,
//         x: usize,
//         y: usize,
//         width: usize,
//         height: usize,
//         data: &[u8],
//     ) -> anyhow::Result<()> {

//         let origin = metal::MTLOrigin { x: x as u64, y: y as u64, z: 0 };
//         let size = metal::MTLSize { width: width as u64, height: height as u64, depth: 0 };
//         let region = metal::MTLRegion { origin, size };

//         let data_offset: usize;
//         let stride: usize;
//         match self.texture_type {
//             nvg::renderer::TextureType::RGBA => {
//                 stride = 4 * self.width();
//                 data_offset = y * stride + x * 4;
//             },
//             nvg::renderer::TextureType::Alpha => {
//                 stride = self.width();
//                 data_offset = y * stride + x;
//             }
//         };
//         self.replace_region(
//             region,
//             0,
//             stride,
//             &data[data_offset..]
//         );

//         Ok(())
//     }
// }
// //impl Drop for Texture {
// //    fn drop(&mut self) {
// //        unsafe { gl::DeleteTextures(1, &self.tex) }
// //    }
// //}
