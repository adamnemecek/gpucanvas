pub fn generate_mipmaps(command_queue: &metal::CommandQueueRef, tex: &metal::TextureRef) {
    let command_buffer = command_queue.new_command_buffer();
    let encoder = command_buffer.new_blit_command_encoder();
    encoder.generate_mipmaps(&tex);

    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();
}

use imgref::ImgVec;
use metalgear::GPUVec;
use rgb::ComponentBytes;
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
            Err(msg) => eprintln!("Cannot save image: {}", msg),
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

pub trait GPUVecExt {
    fn extend_with_triange_fan_indices_cw(&mut self, start: u32, count: u32) -> usize;
}

impl GPUVecExt for GPUVec<u32> {
    /// Creates an indidex buffer which can be used to "fake" triangle fans
    /// Based on pathfinder.
    /// https://www.gamedev.net/forums/topic/643945-how-to-generate-a-triangle-fan-index-list-for-a-circle-shape/

    fn extend_with_triange_fan_indices_cw(&mut self, start: u32, count: u32) -> usize {
        let mut added = 0;
        for index in 1..(count - 1) {
            self.extend_from_slice(&[start, start + index, start + index + 1]);
            added += 3;
        }

        added
    }
}

// fn triangle_fan_indices_cw(start: u32, len: u32) -> Vec<u32> {
//     let mut indices: Vec<u32> = vec![];
//     for index in 1..(len - 1) {
//         indices.extend_from_slice(&[start, start + index, start + index + 1]);
//     }

//     indices
// }
// fn triangle_fan_indices_ccw(start: u32, len: u32) -> Vec<u32> {
//     let mut indices: Vec<u32> = vec![];
//     for index in 1..(len - 1) {
//         indices.extend_from_slice(&[start, start + index + 1, start + index]);
//     }

//     indices
// }

// from https://github.com/OpenSmalltalk/opensmalltalk-vm/blob/4ee8bb6e7960e5776558f0baca10daee7ec5d653/platforms/iOS/plugins/B3DAcceleratorPlugin/sqMetalRenderer.m#L718
// unsigned int triangleCount = vertexCount - 2;
// unsigned int renderIndexCount = triangleCount*3;
// id<MTLBuffer> indexBuffer = [device newBufferWithLength: renderIndexCount*4 options: MTLResourceStorageModeManaged];

// // Set the triangle fan indices.
// unsigned int *destIndices = (unsigned int *)indexBuffer.contents;
// for(unsigned int i = 2; i < vertexCount; ++i) {
//     destIndices[0] = 0;
//     destIndices[1] = i - 1;
//     destIndices[2] = i;
//     destIndices += 3;
// }

// fn triangle_fan_indices2(device: &metal::DeviceRef, start: u32, len: u32) -> GPUVec<u32> {
//     let triangle_len = len - 2;
//     let index_len = 3 * triangle_len;
//     let mut vec = GPUVec::<u32>::with_capacity(device, index_len as usize);

//     // let mut indices: Vec<u32> = vec![];
//     for index in 2..(len) {
//         vec.extend_from_slice(&[start, start + index - 1, start + index]);
//     }

//     vec
// }

// fn triangle_fan_indices3(
//     start: u32,
//     len: u32
// ) -> Vec<u32> {
//     let triangle_len = len - 2;
//     let index_len = 3 * triangle_len;
//     let mut vec = Vec::<u32>::with_capacity(index_len as);

//     // let mut indices: Vec<u32> = vec![];
//     for index in 2..(len) {
//         vec.extend_from_slice(&[start, start + index - 1, start + index]);
//     }

//     vec
// }
// expects buffer to be able to allocate vertices
fn triangle_fan_indices_ext(start: u32, len: usize, buf: &mut GPUVec<u32>) {
    // let mut indices: Vec<u32> = vec![];
    let invariant = buf.capacity();
    // for index in start..(start + len as u32 - 1) {
    //     buf.extend_from_slice(&[start, index, index + 1]);
    // }

    for index in start..(start + len as u32 - 1) {
        buf.extend_from_slice(&[start, index, index + 1]);
    }

    assert!(invariant == buf.capacity());
}

#[cfg(test)]
mod tests {
    use super::GPUVecExt;
    use metalgear::GPUVec;

    #[test]
    fn test_triangle_fan_indices_cw() {
        let expected1: Vec<u32> = vec![0, 1, 2, 0, 2, 3, 0, 3, 4];
        let mut result1 = GPUVec::<u32>::new();
        result1.extend_with_triange_fan_indices_cw(0, 5);
        assert!(expected1[..] == result1[..]);

        let expected2: Vec<u32> = vec![2, 3, 4, 2, 4, 5, 2, 5, 6];
        let mut result2 = GPUVec::<u32>::new();
        result2.extend_with_triange_fan_indices_cw(2, 5);
        assert!(expected2[..] == result2[..]);
    }
}
