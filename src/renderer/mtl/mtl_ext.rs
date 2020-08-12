pub fn generate_mipmaps(command_queue: &metal::CommandQueueRef, tex: &metal::TextureRef) {
    let command_buffer = command_queue.new_command_buffer();
    let encoder = command_buffer.new_blit_command_encoder();
    encoder.generate_mipmaps(&tex);

    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();
}

use rgb::ComponentBytes;
use imgref::ImgVec;
use rgb::RGBA8;


pub trait MtlTextureExt {
    fn save(&self) -> ImgVec<RGBA8>;
    fn save_to(&self, path: &str);
}


impl MtlTextureExt for metal::TextureRef {
    fn save(&self) -> ImgVec<RGBA8> {
        let w = self.width();
        let h = self.height();

        let mut buffer = ImgVec::new(
            vec![
                RGBA8 {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255
                };
                (w * h) as usize
            ],
            w as usize,
            h as usize,
        );

        self.get_bytes(
            buffer.buf_mut().as_ptr() as *mut std::ffi::c_void,
            w * 4,
            metal::MTLRegion {
                origin: metal::MTLOrigin::default(),
                size: metal::MTLSize {
                    width: w,
                    height: h,
                    depth: 1,
                },
            },
            0,
        );

        buffer
    }

    fn save_to(&self, path: &str) {
        let w = self.width();
        let h = self.height();

        let pixel_buf = self.save();

        let fname = path.to_owned();
        match image::save_buffer(
            fname,
            &pixel_buf.buf().as_bytes(),
            w as u32,
            h as u32,
            image::ColorType::Rgba8,
        )
        .map_err(|e| e.to_string())
        {
            Ok(_) => println!("Save complete"),
            Err(msg) => eprintln!("Cannot save blurred image: {}", msg),
        };
    }
}

// pub trait TextureExt {
//     fn size(&self) -> metal::MTLSize;
// }

// impl TextureExt for metal::TextureRef {
//     fn size(&self) -> metal::MTLSize {
//         metal::MTLSize {
//             width: self.width(),
//             height: self.height(),
//             depth: self.depth()
//         }
//     }
// }

// pub trait RenderCommandEncoderExt {
//     fn set_vertex_value<T>(&self, index: u64, value: &T);
//     fn set_fragment_value<T>(&self, index: u64, value: &T);
// }

// impl RenderCommandEncoderExt for metal::RenderCommandEncoderRef {
//     fn set_vertex_value<T>(&self, index: u64, value: &T) {
//         let ptr = value as *const T;
//         self.set_vertex_bytes(index, std::mem::size_of::<T>() as u64, ptr as *const _)
//     }

//     fn set_fragment_value<T>(&self, index: u64, value: &T) {
//         let ptr = value as *const T;
//         self.set_fragment_bytes(index, std::mem::size_of::<T>() as u64, ptr as *const _)
//     }
// }
