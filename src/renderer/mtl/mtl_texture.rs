use crate::{ErrorKind, ImageFlags, ImageInfo, ImageSource, PixelFormat};

use image::{DynamicImage, GenericImageView};
use metal::{
    MTLSamplerAddressMode,
    MTLSamplerMinMagFilter,
    MTLSamplerMipFilter,
    SamplerState,
    Texture,
};

use rgb::ComponentBytes;

pub struct MtlTexture {
    pub info: ImageInfo,
    pub tex: Texture,
    pub sampler: SamplerState,
    // todo: texture has a device reference, use that
    pub device: metal::Device,
}


impl MtlTexture {
    // pub fn pseudo_texture(device: &metal::Device) -> Result<Self, ErrorKind> {
    //     let info = ImageInfo::new(ImageFlags::empty(), 1, 1, PixelFormat::Gray8);
    //     let sampler = metal::SamplerDescriptor::new();

    //     // Self::new(device, info)
    //     todo!()
    // }

    // called renderCreateTextureWithType...
    pub fn new(
        device: &metal::Device,
        command_queue: &metal::CommandQueue,
        info: ImageInfo
    ) -> Result<Self, ErrorKind> {
        // println!("{:?}", info.format());
        let generate_mipmaps = info.flags().contains(ImageFlags::GENERATE_MIPMAPS);
        let nearest = info.flags().contains(ImageFlags::NEAREST);
        let repeatx = info.flags().contains(ImageFlags::REPEAT_X);
        let repeaty = info.flags().contains(ImageFlags::REPEAT_Y);

        let pixel_format = match info.format() {
            PixelFormat::Rgba8 => metal::MTLPixelFormat::R8Unorm,
            PixelFormat::Rgb8 => metal::MTLPixelFormat::R8Unorm,
            PixelFormat::Gray8 => todo!(),
        };

        let desc = metal::TextureDescriptor::new();
        desc.set_texture_type(metal::MTLTextureType::D2);
        desc.set_pixel_format(pixel_format);

        desc.set_width(info.width() as u64);
        desc.set_height(info.height() as u64);

        if generate_mipmaps {
            let size = metal::MTLSize {
                width: info.width() as u64,
                height: info.height() as u64,
                depth: 1
            };
            desc.set_mipmap_level_count_for_size(size);
        } else {
            desc.set_mipmap_level_count(1);
        };

        // todo if macos
        // desc.set_resource_options(metal::MTLResourceOptions::StorageModePrivate);
        desc.set_usage(metal::MTLTextureUsage::RenderTarget |
                        metal::MTLTextureUsage::ShaderWrite |
                        metal::MTLTextureUsage::ShaderRead);

        // let id = alloc_texture_id();
        //let size = src.dimensions();

        // let mut texture = Texture {
        //     id: 0,
        //     info: info
        // };

        // unsafe {
        //     gl::GenTextures(1, &mut texture.id);
        //     gl::BindTexture(gl::TEXTURE_2D, texture.id);
        //     gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        //     gl::PixelStorei(gl::UNPACK_ROW_LENGTH, texture.info.width() as i32);
        //     gl::PixelStorei(gl::UNPACK_SKIP_PIXELS, 0);
        //     gl::PixelStorei(gl::UNPACK_SKIP_ROWS, 0);
        // }

        let data_offset: usize;
        let stride: usize;

        match info.format() {
            PixelFormat::Gray8 => {
                todo!("mtltexture::new grey8");
                // let format = if opengles { gl::LUMINANCE } else { gl::RED };

                // gl::TexImage2D(
                //     gl::TEXTURE_2D,
                //     0,
                //     format as i32,
                //     texture.info.width() as i32,
                //     texture.info.height() as i32,
                //     0,
                //     format,
                //     gl::UNSIGNED_BYTE,
                //     ptr::null()
                //     //data.buf().as_ptr() as *const GLvoid
                // );
            },
            PixelFormat::Rgb8 => {
                todo!()
                // gl::TexImage2D(
                //     gl::TEXTURE_2D,
                //     0,
                //     gl::RGB as i32,
                //     texture.info.width() as i32,
                //     texture.info.height() as i32,
                //     0,
                //     gl::RGB,
                //     gl::UNSIGNED_BYTE,
                //     ptr::null(),
                //     //data.buf().as_ptr() as *const GLvoid
                // );
            },
            PixelFormat::Rgba8 => {
                todo!("mtltexture::new Rgba8");
                // stride = 4 * self.width();
                // data_offset = y * stride + x * 4;
                // gl::TexImage2D(
                //     gl::TEXTURE_2D,
                //     0,
                //     gl::RGBA as i32,
                //     texture.info.width() as i32,
                //     texture.info.height() as i32,
                //     0,
                //     gl::RGBA,
                //     gl::UNSIGNED_BYTE,
                //     ptr::null(),
                //     //data.buf().as_ptr() as *const GLvoid
                // );
            },
        }

        let tex = device.new_texture(&desc);
        let sampler: SamplerState = todo!();

        if generate_mipmaps {
            let command_buffer = command_queue.new_command_buffer();
            let encoder = command_buffer.new_blit_command_encoder();
            encoder.generate_mipmaps(&tex);

            encoder.end_encoding();
            command_buffer.commit();
            command_buffer.wait_until_completed();
        }

        let sampler_desc = metal::SamplerDescriptor::new();

        if nearest {
            sampler_desc.set_min_filter(MTLSamplerMinMagFilter::Nearest);
            sampler_desc.set_mag_filter(MTLSamplerMinMagFilter::Nearest);
            if generate_mipmaps {
                sampler_desc.set_mip_filter(MTLSamplerMipFilter::Nearest);
            }
        } else {
            sampler_desc.set_min_filter(MTLSamplerMinMagFilter::Linear);
            sampler_desc.set_mag_filter(MTLSamplerMinMagFilter::Linear);
            if generate_mipmaps {
                sampler_desc.set_mip_filter(MTLSamplerMipFilter::Linear);
            }
        }

        if repeatx {
            sampler_desc.set_address_mode_s(MTLSamplerAddressMode::Repeat);
        } else {
            sampler_desc.set_address_mode_s(MTLSamplerAddressMode::ClampToEdge);
        }

        if repeaty {
            sampler_desc.set_address_mode_t(MTLSamplerAddressMode::Repeat);
        } else {
            sampler_desc.set_address_mode_t(MTLSamplerAddressMode::ClampToEdge);
        }

        let sampler = device.new_sampler(&sampler_desc);

        Ok(Self {
            info,
            tex,
            sampler,
            device: device.to_owned(),
        })
    }

    // pub fn id(&self) -> u32 {
    //     self.id
    // }

    pub fn replace_region(
        &self,
        region: metal::MTLRegion,
        mipmap_level: usize,
        stride: usize,
        data: &[u8]
    ) {
        self.tex.replace_region(
            region,
            mipmap_level as u64,
            data.as_ptr() as *const _,
            stride as u64,
        )
    }

    pub fn update(&mut self, src: ImageSource, x: usize, y: usize) -> Result<(), ErrorKind> {
        let (width, height) = src.dimensions();
        let origin = metal::MTLOrigin { x: x as u64, y: y as u64, z: 0 };
        let size = metal::MTLSize { width: width as u64, height: height as u64, depth: 0 };
        let region = metal::MTLRegion { origin, size };

        let data_offset: usize;
        let stride: usize;
        let data;

        if x + width > self.info.width() {
            return Err(ErrorKind::ImageUpdateOutOfBounds);
        }

        if y + height > self.info.height() {
            return Err(ErrorKind::ImageUpdateOutOfBounds);
        }

        if self.info.format() != src.format() {
            return Err(ErrorKind::ImageUpdateWithDifferentFormat);
        }

        // unsafe {
        //     gl::BindTexture(gl::TEXTURE_2D, self.id);
        //     gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        //     gl::PixelStorei(gl::UNPACK_ROW_LENGTH, size.0 as i32);
        // }


        match src {
            ImageSource::Gray(data_) => {
                stride = width;
                data_offset = todo!();
                data = data_.buf().as_bytes();
            }
            ImageSource::Rgb(data_) => {
                stride = todo!();
                data_offset = todo!();
                data = data_.buf().as_bytes();
            }
            ImageSource::Rgba(data_) => {
                stride = 4 * width;
                data_offset = y * stride + x;
                data = data_.buf().as_bytes();
            }
        }


        if self.info.flags().contains(ImageFlags::GENERATE_MIPMAPS) {
            todo!()
            // unsafe {
            //     gl::GenerateMipmap(gl::TEXTURE_2D);
            //     //gl::TexParameteri(gl::TEXTURE_2D, gl::GENERATE_MIPMAP, gl::TRUE);
            // }
        }

        self.replace_region(
            region,
            0,
            stride,
            &data[data_offset..]
        );
        Ok(())
    }


    pub fn delete(self) {
        todo!()
        // unsafe {
        //     gl::DeleteTextures(1, &self.id);
        // }
    }

    pub fn info(&self) -> ImageInfo {
        self.info
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