
// pub trait VecExt<T> {
//     fn push_ext(&mut self, value: T) -> usize;
// }

// impl<T> VecExt<T> for Vec<T> {
//     fn push_ext(&mut self, value: T) -> usize {
//         let l = self.len();
//         self.push(value);
//         l
//     }
// }

// impl<T: Copy> VecExt<T> for GPUVec<T> {
//     fn push_ext(&mut self, value: T) -> usize {
//         let l = self.len();
//         self.push(value);
//         l
//     }
// }

// pub struct PathsLength {
//     pub vertex_count: usize,
//     pub index_count: usize,
//     pub stroke_count: usize,
//     pub triangle_count: usize,
// }

// impl PathsLength {
//     pub fn new(cmds: &[Command]) -> Self {
//         let mut vertex_count = 0;
//         let mut index_count = 0;
//         let mut stroke_count = 0;
//         let mut triangle_count = 0;

//         for cmd in cmds {
//             for drawable in &cmd.drawables {
//                 if let Some((_start, count)) = drawable.fill_verts {
//                     if count > 2 {
//                         vertex_count += count;
//                         index_count += (count - 2) * 3;
//                     }
//                 }

//                 if let Some((_start, count)) = drawable.stroke_verts {
//                     if count > 0 {
//                         // todo we shouldn't actually be adding the two since
//                         // the vercies
//                         vertex_count += count + 2;
//                         stroke_count += count;
//                     }
//                 }
//             }

//             if let Some((start, count)) = cmd.triangles_verts {
//                 triangle_count += count;
//             }
//         }

//         Self {
//             vertex_count,
//             index_count,
//             stroke_count,
//             triangle_count,
//         }
//     }
// }

// mod uniform_array;
// use uniform_array::UniformArray;

// #[allow(clippy::all)]
// mod gl {
//     include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
// }

// fn prepare_pipeline_state<'a>(
//     device: &DeviceRef,
//     library: &LibraryRef,
//     vertex_shader: &str,
//     fragment_shader: &str,
// ) -> RenderPipelineState {
//     let vert = library.get_function(vertex_shader, None).unwrap();
//     let frag = library.get_function(fragment_shader, None).unwrap();

//     let pipeline_state_descriptor = RenderPipelineDescriptor::new();
//     pipeline_state_descriptor.set_vertex_function(Some(&vert));
//     pipeline_state_descriptor.set_fragment_function(Some(&frag));
//     let attachment = pipeline_state_descriptor
//         .color_attachments()
//         .object_at(0)
//         .unwrap();
//     attachment.set_pixel_format(MTLPixelFormat::BGRA8Unorm);

//     attachment.set_blending_enabled(true);
//     attachment.set_rgb_blend_operation(metal::MTLBlendOperation::Add);
//     attachment.set_alpha_blend_operation(metal::MTLBlendOperation::Add);
//     attachment.set_source_rgb_blend_factor(metal::MTLBlendFactor::SourceAlpha);
//     attachment.set_source_alpha_blend_factor(metal::MTLBlendFactor::SourceAlpha);
//     attachment.set_destination_rgb_blend_factor(metal::MTLBlendFactor::OneMinusSourceAlpha);
//     attachment.set_destination_alpha_blend_factor(metal::MTLBlendFactor::OneMinusSourceAlpha);

//     device
//         .new_render_pipeline_state(&pipeline_state_descriptor)
//         .unwrap()
// }

// fn main() {
// 	let indices = triangle_fan_indices(10);
//// 	println!("{:?}", indices);
// }
