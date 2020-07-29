use rgb::RGBA8;
use imgref::ImgVec;
use core_graphics::geometry::CGSize;

use crate::{
    Size,
    Color,
    ImageStore,
    ImageSource,
    ImageInfo,
    FillRule,
    CompositeOperationState,
    BlendFactor,
    renderer::{Vertex, ImageId},
    image::ImageFlags,
    ErrorKind
};

use super::{
    Params,
    Renderer,
    Command,
    CommandType,
    RenderTarget
};

mod mtl_texture;
use mtl_texture::MtlTexture;

mod stencil_texture;
use stencil_texture::StencilTexture;

use metalgear::{
    GPUVec,
    GPUVar
};

pub struct PathsLength {
    pub vertex_count: usize,
    pub index_count: usize,
    pub stroke_count: usize,
    pub triangle_count: usize,
}

impl PathsLength {
    pub fn new(cmds: &[Command]) -> Self {
        let mut vertex_count = 0;
        let mut index_count = 0;
        let mut stroke_count = 0;
        let mut triangle_count = 0;

        for cmd in cmds {
            for drawable in &cmd.drawables {
                if let Some((start, count)) = drawable.fill_verts {
                    if count > 2 {
                        vertex_count += count;
                        index_count += (count - 2) * 3;
                    }
                }

                if let Some((start, count)) = drawable.stroke_verts {
                    if count > 0 {
                        vertex_count += count + 2;
                        stroke_count += count;
                    }
                }
            }

            if let Some((start, count)) = cmd.triangles_verts {
                triangle_count += count;
            }
        }

        Self {
            vertex_count,
            index_count,
            stroke_count,
            triangle_count
        }
    }
}


// mod uniform_array;
// use uniform_array::UniformArray;

// #[allow(clippy::all)]
// mod gl {
//     include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
// }


/// Creates an indidex buffer which can be used to "fake" triangle fans
/// Based on pathfinder.
/// https://www.gamedev.net/forums/topic/643945-how-to-generate-a-triangle-fan-index-list-for-a-circle-shape/

fn triangle_fan_indices(quad_len: usize) -> Vec<u32> {
	let mut indices: Vec<u32> = vec![];
	for index in 1..(quad_len as u32 - 1) {
		indices.extend_from_slice(&[0, index as u32, index + 1]);
	}

	indices
}

// fn main() {
// 	let indices = triangle_fan_indices(10);
// 	println!("{:?}", indices);
// }


#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Blend {
    pub src_rgb: metal::MTLBlendFactor,
    pub dst_rgb: metal::MTLBlendFactor,
    pub src_alpha: metal::MTLBlendFactor,
    pub dst_alpha: metal::MTLBlendFactor,
}

fn convert_blend_factor(factor: BlendFactor) -> metal::MTLBlendFactor {
    match factor {
        BlendFactor::Zero => metal::MTLBlendFactor::Zero,
        BlendFactor::One => metal::MTLBlendFactor::One,
        BlendFactor::SrcColor => metal::MTLBlendFactor::SourceColor,
        BlendFactor::OneMinusSrcColor => metal::MTLBlendFactor::OneMinusSourceColor,
        BlendFactor::DstColor => metal::MTLBlendFactor::DestinationColor,
        BlendFactor::OneMinusDstColor => metal::MTLBlendFactor::OneMinusDestinationColor,
        BlendFactor::SrcAlpha => metal::MTLBlendFactor::SourceAlpha,
        BlendFactor::OneMinusSrcAlpha => metal::MTLBlendFactor::OneMinusSourceAlpha,
        BlendFactor::DstAlpha => metal::MTLBlendFactor::DestinationAlpha,
        BlendFactor::OneMinusDstAlpha => metal::MTLBlendFactor::OneMinusDestinationAlpha,
        BlendFactor::SrcAlphaSaturate => metal::MTLBlendFactor::SourceAlphaSaturated,
    }
}


impl From<CompositeOperationState> for Blend {
    fn from(v: CompositeOperationState) -> Self {
        Self {
            src_rgb: convert_blend_factor(v.src_rgb),
            dst_rgb: convert_blend_factor(v.dst_rgb),
            src_alpha: convert_blend_factor(v.src_alpha),
            dst_alpha: convert_blend_factor(v.dst_alpha),
        }
    }
}

// pub struct MTLBuffer {
//     is_busy: bool,
//     image: usize,
//     command_buffer: metal::CommandBuffer,

// }

pub struct Mtl {
    device: metal::Device, // not present in metalnanovg
    // metal has debug and antialias in the flags, opengl
    // has them as properties
    debug: bool,
    antialias: bool,

    command_queue: metal::CommandQueue,
    layer: metal::CoreAnimationLayer,
    // library: metal::Library,
    // render_encoder: Option<metal::RenderCommandEncoder>,

    frag_size: usize,
    index_size: usize,
    // int flags?
    clear_color: Color,
    view_size_buffer: GPUVar<Size>,

    vertex_descriptor: metal::VertexDescriptor,

    blend_func: Blend,
    clear_buffer_on_flush: bool,

    ///
    /// fill and stroke have a stencil, anti_alias_stencil and shape_stencil
    ///
    default_stencil_state: metal::DepthStencilState,
    fill_shape_stencil_state: metal::DepthStencilState,
    fill_anti_alias_stencil_state: metal::DepthStencilState,
    fill_stencil_state: metal::DepthStencilState,
    stroke_shape_stencil_state: metal::DepthStencilState,
    stroke_anti_alias_stencil_state: metal::DepthStencilState,
    stroke_clear_stencil_state: metal::DepthStencilState,

    frag_func: metal::Function,
    vert_func: metal::Function,

    pipeline_pixel_format: metal::MTLPixelFormat,

    pipeline_state: Option<metal::RenderPipelineState>,
    stencil_only_pipeline_state: Option<metal::RenderPipelineState>,

    // these are from mvgbuffer
    stencil_texture: StencilTexture,
    index_buffer: GPUVec<u32>,
    vertex_buffer: GPUVec<Vertex>,
    uniform_buffer: GPUVec<Params>,
    render_target: RenderTarget,
    // todo
    // pseudo_texture: MtlTexture,
    // pseudo_sampler:
}


impl From<CGSize> for Size {
    fn from(v: CGSize) -> Self {
        Self::new(v.width as f32, v.height as f32)
    }
}

pub struct VertexOffsets {
    x: usize,
    u: usize,
}

impl VertexOffsets {
    pub fn new() -> Self {
        use Vertex;
        let x = offset_of!(Vertex, x);
        let u = offset_of!(Vertex, u);
        Self { x, u }
    }
}


impl Mtl {
    pub fn size(&self) -> Size {
        *self.view_size_buffer
    }
}

impl Mtl {
    pub fn new(
        layer: &metal::CoreAnimationLayerRef,
    ) -> Self {

        let device = metal::Device::system_default().unwrap();
        let library_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src/renderer/mtl/shaders.metallib");
        let library = device.new_library_with_file(library_path).expect("library not found");
        let command_queue = device.new_command_queue();

        let debug = true;
        let antialias = true;

        let vert_func = library.get_function("vertexShader", None).expect("vert shader not found");

        let frag_func: metal::Function = if antialias {
            library.get_function("fragmentShader", None).expect("frag shader not found")
        } else {
            library.get_function("fragmentShaderAA", None).expect("frag shader not found")
        };

        let clear_buffer_on_flush = false;

        let drawable_size = layer.drawable_size();

        let vertex_descriptor = {
            let desc = metal::VertexDescriptor::new();
            let offsets = VertexOffsets::new();

            let attrs = desc.attributes().object_at(0).unwrap();
            attrs.set_format(metal::MTLVertexFormat::Float2);
            attrs.set_buffer_index(0);
            attrs.set_offset(offsets.x as u64);

            let attrs = desc.attributes().object_at(1).unwrap();
            attrs.set_format(metal::MTLVertexFormat::Float2);
            attrs.set_buffer_index(0);
            attrs.set_offset(offsets.u as u64);

            let layout = desc.layouts().object_at(0).unwrap();
            layout.set_stride(std::mem::size_of::<Vertex>() as u64);
            layout.set_step_function(metal::MTLVertexStepFunction::PerVertex);
            desc
        };

        // pseudosampler sescriptor
        // let pseudo_texture = MtlTexture::pseudo_texture(&device);
        let stencil_texture = StencilTexture::new(&device, drawable_size.into());

        // Initializes default blend states.
        let blend_func = Blend {
            src_rgb: metal::MTLBlendFactor::One,
            dst_rgb: metal::MTLBlendFactor::OneMinusSourceAlpha,
            src_alpha: metal::MTLBlendFactor::One,
            dst_alpha: metal::MTLBlendFactor::OneMinusSourceAlpha,
        };

        // // Initializes stencil states.
        let stencil_descriptor = metal::DepthStencilDescriptor::new();

        // Default stencil state.
        let default_stencil_state = device.new_depth_stencil_state(&stencil_descriptor);

        // Fill shape stencil.
        let front_face_stencil_descriptor = metal::StencilDescriptor::new();
        front_face_stencil_descriptor.set_stencil_compare_function(metal::MTLCompareFunction::Always);
        front_face_stencil_descriptor.set_depth_stencil_pass_operation(metal::MTLStencilOperation::IncrementWrap);

        let back_face_stencil_descriptor = metal::StencilDescriptor::new();
        back_face_stencil_descriptor.set_stencil_compare_function(metal::MTLCompareFunction::Always);
        back_face_stencil_descriptor.set_depth_stencil_pass_operation(metal::MTLStencilOperation::DecrementWrap);

        stencil_descriptor.set_depth_compare_function(metal::MTLCompareFunction::Always);
        stencil_descriptor.set_back_face_stencil(Some(&back_face_stencil_descriptor));
        stencil_descriptor.set_front_face_stencil(Some(&front_face_stencil_descriptor));

        let fill_shape_stencil_state = device.new_depth_stencil_state(&stencil_descriptor);

        // Fill anti-aliased stencil.
        front_face_stencil_descriptor.set_stencil_compare_function(metal::MTLCompareFunction::Equal);
        front_face_stencil_descriptor.set_stencil_failure_operation(metal::MTLStencilOperation::Keep);
        front_face_stencil_descriptor.set_depth_failure_operation(metal::MTLStencilOperation::Keep);
        front_face_stencil_descriptor.set_depth_stencil_pass_operation(metal::MTLStencilOperation::Zero);

        stencil_descriptor.set_back_face_stencil(None);
        stencil_descriptor.set_front_face_stencil(Some(&front_face_stencil_descriptor));
        let fill_anti_alias_stencil_state = device.new_depth_stencil_state(&stencil_descriptor);

        // Fill stencil.
        front_face_stencil_descriptor.set_stencil_compare_function(metal::MTLCompareFunction::NotEqual);
        front_face_stencil_descriptor.set_stencil_failure_operation(metal::MTLStencilOperation::Zero);
        front_face_stencil_descriptor.set_depth_failure_operation(metal::MTLStencilOperation::Zero);
        front_face_stencil_descriptor.set_depth_stencil_pass_operation(metal::MTLStencilOperation::Zero);

        stencil_descriptor.set_back_face_stencil(None);
        stencil_descriptor.set_front_face_stencil(Some(&front_face_stencil_descriptor));
        let fill_stencil_state = device.new_depth_stencil_state(&stencil_descriptor);

        // Stroke shape stencil.
        front_face_stencil_descriptor.set_stencil_compare_function(metal::MTLCompareFunction::Equal);
        front_face_stencil_descriptor.set_stencil_failure_operation(metal::MTLStencilOperation::Keep);
        front_face_stencil_descriptor.set_depth_failure_operation(metal::MTLStencilOperation::Keep);
        front_face_stencil_descriptor.set_depth_stencil_pass_operation(metal::MTLStencilOperation::IncrementClamp);

        stencil_descriptor.set_back_face_stencil(None);
        stencil_descriptor.set_front_face_stencil(Some(&front_face_stencil_descriptor));
        let stroke_shape_stencil_state = device.new_depth_stencil_state(&stencil_descriptor);

        // Stroke anti-aliased stencil.
        front_face_stencil_descriptor.set_depth_stencil_pass_operation(metal::MTLStencilOperation::Keep);

        stencil_descriptor.set_back_face_stencil(None);
        stencil_descriptor.set_front_face_stencil(Some(&front_face_stencil_descriptor));
        let stroke_anti_alias_stencil_state = device.new_depth_stencil_state(&stencil_descriptor);

        // Stroke clear stencil.
        front_face_stencil_descriptor.set_stencil_compare_function(metal::MTLCompareFunction::Always);
        front_face_stencil_descriptor.set_stencil_failure_operation(metal::MTLStencilOperation::Zero);
        front_face_stencil_descriptor.set_depth_failure_operation(metal::MTLStencilOperation::Zero);
        front_face_stencil_descriptor.set_depth_stencil_pass_operation(metal::MTLStencilOperation::Zero);

        stencil_descriptor.set_back_face_stencil(None);
        stencil_descriptor.set_front_face_stencil(Some(&front_face_stencil_descriptor));

        let stroke_clear_stencil_state = device.new_depth_stencil_state(&stencil_descriptor);

        Self {
            layer: layer.to_owned(),
            debug,
            antialias,
            blend_func,
            // render_encoder: None,
            // todo check what is this initialized to
            view_size_buffer: GPUVar::with_value(&device, Size::default()),
            command_queue,
            frag_func,
            vert_func,
            pipeline_state: None,
            clear_buffer_on_flush,
            default_stencil_state,
            fill_shape_stencil_state,
            fill_anti_alias_stencil_state,
            fill_stencil_state,
            stroke_shape_stencil_state,
            stroke_anti_alias_stencil_state,
            stroke_clear_stencil_state,
            frag_size: std::mem::size_of::<Params>(),
            index_size: 4, // MTLIndexTypeUInt32
            stencil_only_pipeline_state: None,
            stencil_texture,
            index_buffer: GPUVec::with_capacity(&device, 32),
            vertex_buffer: GPUVec::with_capacity(&device, 32),
            uniform_buffer: GPUVec::with_capacity(&device, 2),
            vertex_descriptor: vertex_descriptor.to_owned(),
            pipeline_pixel_format: metal::MTLPixelFormat::Invalid,
            render_target: RenderTarget::Screen,
            // pseudo_texture: pseudo_texture.unwrap(),
            clear_color: Color::black(),
            device,
        }
    }

    // fn factor(factor: BlendFactor) -> metal::MTLBlendFactor {
    //     use metal::MTLBlendFactor;

    //     match factor {
    //         BlendFactor::Zero => MTLBlendFactor::Zero,
    //         BlendFactor::One => MTLBlendFactor::One,
    //         BlendFactor::SrcColor => MTLBlendFactor::SourceColor,
    //         BlendFactor::OneMinusSrcColor => MTLBlendFactor::OneMinusSourceColor,
    //         BlendFactor::DstColor => MTLBlendFactor::DestinationColor,
    //         BlendFactor::OneMinusDstColor => MTLBlendFactor::OneMinusDestinationColor,
    //         BlendFactor::SrcAlpha => MTLBlendFactor::SourceAlpha,
    //         BlendFactor::OneMinusSrcAlpha => MTLBlendFactor::OneMinusSourceAlpha,
    //         BlendFactor::DstAlpha => MTLBlendFactor::DestinationAlpha,
    //         BlendFactor::OneMinusDstAlpha => MTLBlendFactor::OneMinusDestinationAlpha,
    //         BlendFactor::SrcAlphaSaturate => MTLBlendFactor::SourceAlphaSaturated,
    //     }
    // }



    /// updaterenderpipelinstateforblend
    pub fn set_composite_operation(
        &mut self,
        blend_func: CompositeOperationState,
        pixel_format: metal::MTLPixelFormat
    ) {
        let blend_func: Blend = blend_func.into();

        if self.pipeline_state.is_some() &&
            self.stencil_only_pipeline_state.is_some() &&
            self.pipeline_pixel_format == pixel_format &&
            self.blend_func == blend_func {
            return;
        }

        let desc = metal::RenderPipelineDescriptor::new();
        let color_attachment_desc = desc.color_attachments().object_at(0).unwrap();
        color_attachment_desc.set_pixel_format(pixel_format);

        desc.set_stencil_attachment_pixel_format(metal::MTLPixelFormat::Stencil8);
        desc.set_fragment_function(Some(&self.frag_func));
        desc.set_vertex_function(Some(&self.vert_func));
        desc.set_vertex_descriptor(Some(&self.vertex_descriptor));

        color_attachment_desc.set_blending_enabled(true);
        color_attachment_desc.set_source_rgb_blend_factor(blend_func.src_rgb);
        color_attachment_desc.set_source_alpha_blend_factor(blend_func.src_alpha);
        color_attachment_desc.set_destination_rgb_blend_factor(blend_func.dst_rgb);
        color_attachment_desc.set_destination_alpha_blend_factor(blend_func.dst_alpha);

        self.blend_func = blend_func;
        let pipeline_state = self.device.new_render_pipeline_state(&desc).unwrap();
        self.pipeline_state = Some(pipeline_state);

        desc.set_fragment_function(None);
        color_attachment_desc.set_write_mask(metal::MTLColorWriteMask::empty());
        let stencil_only_pipeline_state = self.device.new_render_pipeline_state(&desc).unwrap();
        self.stencil_only_pipeline_state = Some(stencil_only_pipeline_state);

        // self.pipeline_pixel_format = pixel_format;
//         unsafe {
//             gl::BlendFuncSeparate(
//                 Self::gl_factor(blend_state.src_rgb),
//                 Self::gl_factor(blend_state.dst_rgb),
//                 Self::gl_factor(blend_state.src_alpha),
//                 Self::gl_factor(blend_state.dst_alpha)
//             );
//         }
        // self.pipeline
    }

    /// done
    pub fn convex_fill(
        &mut self,
        encoder: &metal::RenderCommandEncoderRef,
        images: &ImageStore<MtlTexture>,
        cmd: &Command,
        gpu_paint: Params
    ) {
        self.set_uniforms(encoder, images, gpu_paint, cmd.image, cmd.alpha_mask);
        encoder.set_render_pipeline_state(&self.pipeline_state.as_ref().unwrap());

        for drawable in &cmd.drawables {
            if let Some((start, count)) = drawable.fill_verts {
                /// offset is in bytes
                let index_buffer_offset = start * self.index_size;

                /// original uses fans
                // encoder.draw_indexed_primitives(
                //     metal::MTLPrimitiveType::Triangle,
                //     count as u64,
                //     metal::MTLIndexType::UInt32,
                //     self.index_buffer.as_ref(),
                //     index_buffer_offset as u64,
                // );
                todo!()
            }

             // Draw fringes
            if let Some((start, count)) = drawable.stroke_verts {
                encoder.draw_primitives(
                    metal::MTLPrimitiveType::TriangleStrip,
                    start as u64,
                    count as u64
                )
            }
        }
    }

    /// done
    pub fn concave_fill(
        &mut self,
        encoder: &metal::RenderCommandEncoderRef,
        images: &ImageStore<MtlTexture>,
        cmd: &Command,
        stencil_paint: Params,
        fill_paint: Params
    ) {
        encoder.set_cull_mode(metal::MTLCullMode::None);
        encoder.set_depth_stencil_state(&self.fill_shape_stencil_state);
        encoder.set_render_pipeline_state(&self.stencil_only_pipeline_state.as_ref().unwrap());

        /// todo metal nanovg doesn't have this
        self.set_uniforms(encoder, images, stencil_paint, None, None);

        for drawable in &cmd.drawables {
            if let Some((start, count)) = drawable.fill_verts {
                /// offset is in bytes
                let index_buffer_offset = start * self.index_size;

                /// draw fans
                todo!()
                // encoder.draw_indexed_primitives(
                //     metal::MTLPrimitiveType::Triangle,
                //     count as u64,
                //     metal::MTLIndexType::UInt32,
                //     self.index_buffer.as_ref(),
                //     index_buffer_offset as u64,
                // );
            }
        }
        // Restores states.
        encoder.set_cull_mode(metal::MTLCullMode::Back);
        encoder.set_render_pipeline_state(&self.pipeline_state.as_ref().unwrap());

        // Draws anti-aliased fragments.
        self.set_uniforms(encoder, images, fill_paint, cmd.image, cmd.alpha_mask);
        if self.antialias {
            encoder.set_depth_stencil_state(&self.fill_anti_alias_stencil_state);

            for drawable in &cmd.drawables {
                if let Some((start, count)) = drawable.stroke_verts {
                    /// draw fans
                    encoder.draw_primitives(
                        metal::MTLPrimitiveType::TriangleStrip,
                        start as u64,
                        count as u64
                    );
                }
            }
        }

        // Draws fill.
        encoder.set_depth_stencil_state(&self.fill_stencil_state);
        if let Some((start, count)) = cmd.triangles_verts {
            encoder.draw_primitives(
                metal::MTLPrimitiveType::TriangleStrip,
                start as u64,
                count as u64
            );
        }
        encoder.set_depth_stencil_state(&self.default_stencil_state);
    }

    /// done
    pub fn stroke(
        &mut self,
        encoder: &metal::RenderCommandEncoderRef,
        images: &ImageStore<MtlTexture>,
        cmd: &Command,
        paint: Params
    ) {
        self.set_uniforms(encoder, images, paint, cmd.image, cmd.alpha_mask);
        for drawable in &cmd.drawables {
            if let Some((start, count)) = drawable.stroke_verts {
                encoder.draw_primitives(
                    metal::MTLPrimitiveType::TriangleStrip,
                    start as u64,
                    count as u64
                )
            }
        }
    }

    /// done
    pub fn stencil_stroke(
        &mut self,
        encoder: &metal::RenderCommandEncoderRef,
        images: &ImageStore<MtlTexture>,
        cmd: &Command,
        paint1: Params,
        paint2: Params
    ) {
        /// Fills the stroke base without overlap.
        self.set_uniforms(encoder, images, paint2, cmd.image, cmd.alpha_mask);
        encoder.set_depth_stencil_state(&self.stroke_shape_stencil_state);
        encoder.set_render_pipeline_state(&self.pipeline_state.as_ref().unwrap());

        for drawable in &cmd.drawables {
            if let Some((start, count)) = drawable.stroke_verts {
                encoder.draw_primitives(
                    metal::MTLPrimitiveType::TriangleStrip,
                    start as u64,
                    count as u64
                )
            }
        }

        /// Draw anti-aliased pixels.
        self.set_uniforms(encoder, images, paint1, cmd.image, cmd.alpha_mask);
        encoder.set_depth_stencil_state(&self.stroke_anti_alias_stencil_state);

        for drawable in &cmd.drawables {
            if let Some((start, count)) = drawable.stroke_verts {
                // unsafe { gl::DrawArrays(gl::TRIANGLE_STRIP, start as i32, count as i32); }
                encoder.draw_primitives(
                    metal::MTLPrimitiveType::TriangleStrip,
                    start as u64,
                    count as u64
                );
            }
        }

        /// Clears stencil buffer.
        encoder.set_depth_stencil_state(&self.stroke_clear_stencil_state);
        encoder.set_render_pipeline_state(&self.stencil_only_pipeline_state.as_ref().unwrap());

        for drawable in &cmd.drawables {
            if let Some((start, count)) = drawable.stroke_verts {
                encoder.draw_primitives(
                    metal::MTLPrimitiveType::TriangleStrip,
                    start as u64,
                    count as u64
                );
            }
        }
        encoder.set_depth_stencil_state(&self.default_stencil_state);
    }

    /// done
    pub fn triangles(
        &mut self,
        encoder: &metal::RenderCommandEncoderRef,
        images: &ImageStore<MtlTexture>,
        cmd: &Command,
        paint: Params
    ) {
        self.set_uniforms(encoder, images, paint, cmd.image, cmd.alpha_mask);
        encoder.set_render_pipeline_state(&self.pipeline_state.as_ref().unwrap());
        if let Some((start, count)) = cmd.triangles_verts {
            encoder.draw_primitives(
                metal::MTLPrimitiveType::Triangle,
                start as u64,
                count as u64
            );
        }
    }

    pub fn set_uniforms(
        &mut self,
        encoder: &metal::RenderCommandEncoderRef,
        images: &ImageStore<MtlTexture>,
        paint: Params,
        image_tex: Option<ImageId>,
        alpha_tex: Option<ImageId>
    ) {
        ///
        /// https://developer.apple.com/documentation/metal/mtlrendercommandencoder/1515917-setfragmentbufferoffset?language=objc
        ///
        let len = self.uniform_buffer.len();
        self.uniform_buffer.push(paint);
        let offset = len * std::mem::size_of::<Params>();
        encoder.set_fragment_buffer_offset(0, offset as u64);


        let tex = if let Some(id) = image_tex {
            images.get(id).unwrap()
        }
        else {
            // &self.pseudo_texture
            todo!()
        };

        encoder.set_fragment_texture(0, Some(&tex.tex));
        encoder.set_fragment_sampler_state(0, Some(&tex.sampler));

//         unsafe {
//             gl::ActiveTexture(gl::TEXTURE0);
//             gl::BindTexture(gl::TEXTURE_2D, tex);
//         }

//         let masktex = alpha_tex.and_then(|id| images.get(id)).map_or(0, |tex| tex.id());

//         unsafe {
//             gl::ActiveTexture(gl::TEXTURE0 + 1);
//             gl::BindTexture(gl::TEXTURE_2D, masktex);
//         }

//         self.check_error("set_uniforms texture");
    }

    // from warrenmoore
    // Well, as I think we discussed previously, scissor state doesn’t affect clear load actions in Metal, but you can simulate this by drawing a rect with a solid color with depth read disabled and depth write enabled and forcing the depth to the clear depth value (assuming you’re using a depth buffer)
    // Looks like in this case the depth buffer is irrelevant. Stencil buffer contents can be cleared similarly to the depth buffer, though

    // mnvgclearwithcolor
    pub fn clear_rect(
        &mut self,
        encoder: &metal::RenderCommandEncoderRef,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        color: Color) {
            self.clear_color = color;
            // self.clear_color = metal::MTLClearColor::new(color.r, color.g, color.b, color.a);
            self.clear_buffer_on_flush = true;
            // todo!()
            // let scissor_rect: metal::MTLScissorRect = todo!();
            // encoder.set_viewport(viewport);
//         unsafe {
//             gl::Enable(gl::SCISSOR_TEST);
//             gl::Scissor(x as i32, self.view[1] as i32 - (height as i32 + y as i32), width as i32, height as i32);
//             gl::ClearColor(color.r, color.g, color.b, color.a);
//             gl::Clear(gl::COLOR_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
//             gl::Disable(gl::SCISSOR_TEST);
//         }
    }
    fn set_target(&mut self, images: &ImageStore<MtlTexture>, target: RenderTarget) {
        todo!()
    }

    // pub fn get_target(&self, images: &ImageStore<MtlTexture>) -> metal::TextureRef {
    //     match self.render_target {
    //         RenderTarget::Screen => {
    //             todo!()
    //         },
    //         RenderTarget::Image(id) => {
    //             todo!()
    //         }
    //     }
    // }

    // pub fn reset(&mut self) {

    // }

    // pub fn new_command_buffer(&self) -> metal::CommandBuffer {
    //     self.command_queue.new_command_buffer().to_owned()
    // }

    // pub fn clear_color(&self) -> Color {
    //     self.clear_color
    // }
}

impl From<Color> for metal::MTLClearColor {
    fn from(v: Color) -> Self {
        Self::new(v.r.into(), v.g.into(), v.b.into(), v.a.into())
    }
}

fn new_render_command_encoder<'a>(
    color_texture: &metal::TextureRef,
    command_buffer: &'a metal::CommandBufferRef,
    clear_color: Color,
    stencil_texture: &StencilTexture,
    // view_size: Size,
    vertex_buffer: &GPUVec<Vertex>,
    view_size_buffer: &GPUVar<Size>,
    // index_buffer: &IndexBuffer,
    uniform_buffer: &GPUVec<Params>,
    clear_buffer_on_flush: bool
) -> &'a metal::RenderCommandEncoderRef {

    let load_action = if clear_buffer_on_flush {
        metal::MTLLoadAction::Clear
    } else {
        metal::MTLLoadAction::Load
    };
    let desc = metal::RenderPassDescriptor::new();

    let view_size = &*view_size_buffer;

    let color_attachment = desc.color_attachments().object_at(0).unwrap();
    color_attachment.set_clear_color(clear_color.into());
    color_attachment.set_load_action(load_action);
    color_attachment.set_store_action(metal::MTLStoreAction::Store);
    color_attachment.set_texture(Some(&color_texture));

    let stencil_attachment = desc.stencil_attachment().unwrap();
    stencil_attachment.set_clear_stencil(0);
    stencil_attachment.set_load_action(metal::MTLLoadAction::Clear);
    stencil_attachment.set_store_action(metal::MTLStoreAction::DontCare);
    stencil_attachment.set_texture(Some(&stencil_texture.tex));

    let encoder = command_buffer.new_render_command_encoder(&desc);

    encoder.set_cull_mode(metal::MTLCullMode::Back);
    encoder.set_front_facing_winding(metal::MTLWinding::Clockwise);
    encoder.set_stencil_reference_value(0);
    encoder.set_viewport(metal::MTLViewport {
        originX: 0.0,
        originY: 0.0,
        width: view_size.w as f64,
        height: view_size.h as f64,
        znear: 0.0,
        zfar: 1.0,
    });

    encoder.set_vertex_buffer(0, Some(vertex_buffer.as_ref()), 0);
    encoder.set_vertex_buffer(1, Some(view_size_buffer.as_ref()), 0);
    encoder.set_fragment_buffer(0, Some(uniform_buffer.as_ref()), 0);

    encoder
}

impl Renderer for Mtl {
    type Image = MtlTexture;

    fn set_size(&mut self, width: u32, height: u32, dpi: f32) {
        let size = Size::new(width as f32, height as f32);
        *self.view_size_buffer = size;
    }

    // called flush in ollix and nvg
    fn render(&mut self, images: &ImageStore<Self::Image>, verts: &[Vertex], commands: &[Command]) {
        // let lens = PathsLength::new(commands);

        self.vertex_buffer.clear();
        self.vertex_buffer.extend_from_slice(verts);

        /// build indices
        self.index_buffer.clear();

        let clear_color: Color = self.clear_color;

        let command_buffer = self.command_queue.new_command_buffer().to_owned();
        command_buffer.enqueue();

        // todo: this should be calling get_target
        let drawable = self.layer.next_drawable().unwrap().to_owned();
        let color_texture = drawable.texture();
        let pixel_format = color_texture.pixel_format();

        let encoder = new_render_command_encoder(
            &color_texture,
            &command_buffer,
            clear_color,
            &self.stencil_texture,
            &self.vertex_buffer,
            &self.view_size_buffer,
            &self.uniform_buffer,
            self.clear_buffer_on_flush,
        );
        self.clear_buffer_on_flush = false;

        for cmd in commands {
            self.set_composite_operation(cmd.composite_operation, pixel_format);

            match cmd.cmd_type {
                CommandType::ConvexFill { params } => {
                    self.convex_fill(&encoder, images, cmd, params)
                },
                CommandType::ConcaveFill { stencil_params, fill_params } => {
                    self.concave_fill(&encoder, images, cmd, stencil_params, fill_params)
                },
                CommandType::Stroke { params } => {
                    self.stroke(&encoder, images, cmd, params)
                },
                CommandType::StencilStroke { params1, params2 } => {
                    self.stencil_stroke(&encoder, images, cmd, params1, params2)
                },
                CommandType::Triangles { params } => {
                    self.triangles(&encoder, images, cmd, params)
                },
                CommandType::ClearRect { x, y, width, height, color } => {
                    self.clear_rect(&encoder, x, y, width, height, color);
                }
                CommandType::SetRenderTarget(target) => {
                    self.set_target(images, target);

                }
            }
        }

        encoder.end_encoding();
        if !self.layer.presents_with_transaction() {
            command_buffer.present_drawable(&drawable);
        }

        #[cfg(target_os = "macos")]
        {
            let blit = command_buffer.new_blit_command_encoder();
            blit.synchronize_resource(color_texture);
            blit.end_encoding();
        }

        if self.layer.presents_with_transaction() {
            command_buffer.wait_until_scheduled();
            drawable.present();
        }
    }

    fn alloc_image(&mut self, info: ImageInfo) -> Result<Self::Image, ErrorKind> {
        MtlTexture::new(&self.device, info)
    }

    fn update_image(&mut self, image: &mut Self::Image, data: ImageSource, x: usize, y: usize) ->  Result<(), ErrorKind> {
        image.update(data, x, y)
    }

    fn delete_image(&mut self, image: Self::Image) {
        image.delete();
    }

    // fn set_target(&mut self, images: &ImageStore<MtlTexture>, target: RenderTarget) {
    //     self.render_target = target;
    //     todo!();
    // }

    fn screenshot(&mut self) -> Result<ImgVec<RGBA8>, ErrorKind> {
        // todo!()
        let size = *self.view_size_buffer;
        let w = size.w as usize;
        let h = size.h as usize;

        let mut image = ImgVec::new(vec![RGBA8 {r:255, g:255, b:255, a: 255}; w*h], w, h);
        todo!()
        // unsafe {
        //     gl::ReadPixels(0, 0, self.view[0] as i32, self.view[1] as i32, gl::RGBA, gl::UNSIGNED_BYTE, image.buf_mut().as_ptr() as *mut GLvoid);
        // }
        // todo!()
        // TODO: flip image
        //image = image::imageops::flip_vertical(&image);

        // Ok(image)
    }
}


mod tests {
    use super::triangle_fan_indices;

    #[test]
    fn test_triangle_fan_indices() {
        let expected: Vec<u32> = vec![0, 1, 2, 0, 2, 3, 0, 3, 4];
        let result = triangle_fan_indices(5);
        assert!(expected == result);

    }
}