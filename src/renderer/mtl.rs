use imgref::ImgVec;
use metal::CGSize;
use rgb::RGBA8;

use metalgear::{GPUVar, GPUVec, RenderCommandEncoderExt};

//use super::mtl::mtl_ext::RenderCommandEncoderExt;
use super::{Command, CommandType, Params, RenderTarget, Renderer};
use crate::{
    image::ImageFlags,
    renderer::{ImageId, Vertex},
    BlendFactor, Color, CompositeOperationState, ErrorKind, FillRule, ImageInfo, ImageSource, ImageStore, Rect, Size,
};

mod mtl_texture;
pub use mtl_texture::MtlTexture;

mod rps_cache;
pub use rps_cache::{RPSCache, RPS};

mod stencil_texture;
use stencil_texture::StencilTexture;

mod mtl_ext;
pub use mtl_ext::{GPUVecExt, MtlTextureExt};

mod buffers_cache;
pub use buffers_cache::*;

// lazy_static! {
//     static ref BUFFER_CACHE: BuffersCache = BuffersCache::new(&metal::Device::system_default().unwrap(), 3);
//     // static ref NUMBER: u32 = times_two(21);
// }

#[repr(C)]
#[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd)]
struct ClearRect {
    rect: Rect,
    color: Color,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Blend {
    pub src_rgb: metal::MTLBlendFactor,
    pub dst_rgb: metal::MTLBlendFactor,
    pub src_alpha: metal::MTLBlendFactor,
    pub dst_alpha: metal::MTLBlendFactor,
}

impl From<BlendFactor> for metal::MTLBlendFactor {
    fn from(a: BlendFactor) -> Self {
        match a {
            BlendFactor::Zero => Self::Zero,
            BlendFactor::One => Self::One,
            BlendFactor::SrcColor => Self::SourceColor,
            BlendFactor::OneMinusSrcColor => Self::OneMinusSourceColor,
            BlendFactor::DstColor => Self::DestinationColor,
            BlendFactor::OneMinusDstColor => Self::OneMinusDestinationColor,
            BlendFactor::SrcAlpha => Self::SourceAlpha,
            BlendFactor::OneMinusSrcAlpha => Self::OneMinusSourceAlpha,
            BlendFactor::DstAlpha => Self::DestinationAlpha,
            BlendFactor::OneMinusDstAlpha => Self::OneMinusDestinationAlpha,
            BlendFactor::SrcAlphaSaturate => Self::SourceAlphaSaturated,
        }
    }
}

impl From<CompositeOperationState> for Blend {
    fn from(v: CompositeOperationState) -> Self {
        Self {
            src_rgb: v.src_rgb.into(),
            dst_rgb: v.dst_rgb.into(),
            src_alpha: v.src_alpha.into(),
            dst_alpha: v.dst_alpha.into(),
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
    layer: metal::MetalLayer,
    // library: metal::Library,
    // render_encoder: Option<metal::RenderCommandEncoder>,
    frag_size: usize,
    index_size: usize,
    // int flags?
    clear_color: Color,
    // view_size_buffer: GPUVar<Size>,
    view_size: Size,
    multiple_buffering: usize,
    // screen_view: [f32; 2],
    // vertex_descriptor: metal::VertexDescriptor,

    // blend_func: Blend,
    // clear_buffer_on_flush: bool,
    //
    // fill and stroke have a stencil, anti_alias_stencil and shape_stencil
    //
    default_stencil_state: metal::DepthStencilState,
    fill_shape_stencil_state: metal::DepthStencilState,
    fill_anti_alias_stencil_state: metal::DepthStencilState,
    fill_stencil_state: metal::DepthStencilState,
    stroke_shape_stencil_state: metal::DepthStencilState,
    stroke_anti_alias_stencil_state: metal::DepthStencilState,
    stroke_clear_stencil_state: metal::DepthStencilState,

    rps_cache: RPSCache,
    current_rps: Option<RPS>,
    // vert_func: metal::Function,
    // frag_func: metal::Function,

    // pipeline_pixel_format: metal::MTLPixelFormat,

    // pipeline_state: Option<metal::RenderPipelineState>,
    // stencil_only_pipeline_state: Option<metal::RenderPipelineState>,

    // these are from mvgbuffer
    stencil_texture: StencilTexture,
    index_buffer: GPUVec<u32>,
    vertex_buffer: GPUVec<Vertex>,
    // uniform_buffer: GPUVec<Params>,
    render_target: RenderTarget,

    // todo
    pseudo_texture: MtlTexture,
    // buffers_cache: MtlBuffersCache,

    // // we render into this texture and blit with into the target texture
    // // as opposed to the target texture directly in order to avoid creating
    // // multiple encoders
    // temp_texture: MtlTexture,
    // pseudo_sampler:

    // clear_rect
    // clear_rect_vert_func: metal::Function,
    // clear_rect_frag_func: metal::Function,
    // clear_rect_pipeline_state: Option<metal::RenderPipelineState>,

    // Needed for screenshoting,
    //
    // last_rendered_texture: Option<metal::Texture>,
    frame: usize,
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
        // use Vertex;
        let x = offset_of!(Vertex, x);
        let u = offset_of!(Vertex, u);
        Self { x, u }
    }
}

impl Mtl {
    pub fn size(&self) -> Size {
        // *self.view_size_buffer
        self.view_size
    }
}

impl Mtl {
    pub fn new(
        device: &metal::DeviceRef,
        command_queue: &metal::CommandQueueRef,
        layer: &metal::MetalLayerRef,
    ) -> Self {
        let debug = cfg!(debug_assertions);
        let antialias = true;

        #[cfg(target_os = "macos")]
        {
            layer.set_opaque(false);
        }

        // let root_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        // let library_path = root_path.join("src/renderer/mtl/shaders.metallib");
        // let library = device.new_library_with_file(library_path).expect("library not found");
        // let command_queue = device.new_command_queue();
        let command_queue = command_queue.to_owned();
        let rps_cache = RPSCache::new(device, antialias);

        // let vert_func = library
        //     .get_function("vertexShader", None)
        //     .expect("vert shader not found");

        // let frag_func: metal::Function = if antialias {
        //     library
        //         .get_function("fragmentShaderAA", None)
        //         .expect("frag shader not found")
        // } else {
        //     library
        //         .get_function("fragmentShader", None)
        //         .expect("frag shader not found")
        // };

        // let clear_rect_vert_func = library
        //     .get_function("clear_rect_vertex", None)
        //     .expect("clear_rect_vertex shader not found");

        // let clear_rect_frag_func = library
        //     .get_function("clear_rect_fragment", None)
        //     .expect("clear_rect_fragment shader not found");

        // let clear_buffer_on_flush = false;

        let drawable_size = layer.drawable_size();
        let size: Size = drawable_size.into();

        // let vertex_descriptor = {
        //     let desc = metal::VertexDescriptor::new();
        //     let offsets = VertexOffsets::new();

        //     let attrs = desc.attributes().object_at(0).unwrap();
        //     attrs.set_format(metal::MTLVertexFormat::Float2);
        //     attrs.set_buffer_index(0);
        //     attrs.set_offset(offsets.x as u64);

        //     let attrs = desc.attributes().object_at(1).unwrap();
        //     attrs.set_format(metal::MTLVertexFormat::Float2);
        //     attrs.set_buffer_index(0);
        //     attrs.set_offset(offsets.u as u64);

        //     let layout = desc.layouts().object_at(0).unwrap();
        //     layout.set_stride(std::mem::size_of::<Vertex>() as u64);
        //     layout.set_step_function(metal::MTLVertexStepFunction::PerVertex);
        //     desc
        // };

        // pseudosampler sescriptor
        let pseudo_texture = MtlTexture::new_pseudo_texture(device, &command_queue).unwrap();
        pseudo_texture.set_label("pseudo_texture");
        let stencil_texture = StencilTexture::new(&device, drawable_size.into());
        stencil_texture.set_label("stencil_texture");

        // Initializes default blend states.
        // let blend_func = Blend {
        //     src_rgb: metal::MTLBlendFactor::One,
        //     dst_rgb: metal::MTLBlendFactor::OneMinusSourceAlpha,
        //     src_alpha: metal::MTLBlendFactor::One,
        //     dst_alpha: metal::MTLBlendFactor::OneMinusSourceAlpha,
        // };

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
        // we set only 
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

        let image_info = ImageInfo::new(ImageFlags::empty(), size.w as _, size.h as _, crate::PixelFormat::Rgba8);
        // let temp_texture = MtlTexture::new(device, &command_queue, image_info).unwrap();

        Self {
            // temp_texture,
            multiple_buffering: 3,
            layer: layer.to_owned(),
            // buffers_cache: MtlBuffersCache::new(&device, 3),
            debug,
            antialias,
            rps_cache,
            current_rps: None,
            // blend_func,
            // render_encoder: None,
            // todo check what is this initialized to
            // view_size_buffer: GPUVar::with_value(&device, size),
            view_size: size,
            command_queue,
            // frag_func,
            // vert_func,
            // pipeline_state: None,
            // screen_view: [0.0, 0.0],
            // clear_buffer_on_flush,
            default_stencil_state,
            fill_shape_stencil_state,
            fill_anti_alias_stencil_state,
            fill_stencil_state,
            stroke_shape_stencil_state,
            stroke_anti_alias_stencil_state,
            stroke_clear_stencil_state,
            frag_size: std::mem::size_of::<Params>(),
            index_size: 4, // MTLIndexTypeUInt32
            // stencil_only_pipeline_state: None,
            stencil_texture,
            index_buffer: GPUVec::with_capacity(&device, 32),
            vertex_buffer: GPUVec::with_capacity(&device, 32),
            // uniform_buffer: GPUVec::with_capacity(&device, 2),
            // vertex_descriptor: vertex_descriptor.to_owned(),
            // pipeline_pixel_format: metal::MTLPixelFormat::Invalid,
            render_target: RenderTarget::Screen,
            pseudo_texture,
            clear_color: Color::blue(),
            device: device.to_owned(),

            // clear_rect_vert_func,
            // clear_rect_frag_func,
            // clear_rect_pipeline_state: None,
            // last_rendered_texture: None,
            frame: 0,
        }
    }

    /// updaterenderpipelinstateforblend
    pub fn set_composite_operation(
        &mut self,
        blend_func: CompositeOperationState,
        pixel_format: metal::MTLPixelFormat,
    ) {
        // println!("set_composite operation {:?}", pixel_format);
        let blend_func: Blend = blend_func.into();
        if let Some(current_rps) = self.current_rps.as_ref() {
            if current_rps.blend_func == blend_func && current_rps.pixel_format == pixel_format {
                return;
            }
        }

        self.current_rps = Some(self.rps_cache.get(blend_func, pixel_format));

        // println!("rps_cache.len() == {}", self.rps_cache.len());
        // if self.pipeline_state.is_some()
        //     && self.stencil_only_pipeline_state.is_some()
        //     && self.pipeline_pixel_format == pixel_format
        //     && self.blend_func == blend_func
        // {
        //     // println!("skipping setting composite op");
        //     return;
        // }
        // println!("setting composite op for {:?} and {:?}", blend_func, pixel_format);

        // let desc = metal::RenderPipelineDescriptor::new();
        // let color_attachment_desc = desc.color_attachments().object_at(0).unwrap();
        // color_attachment_desc.set_pixel_format(pixel_format);

        // // println!("blend: {:?}", blend_func);
        // desc.set_stencil_attachment_pixel_format(metal::MTLPixelFormat::Stencil8);
        // desc.set_vertex_function(Some(&self.vert_func));
        // desc.set_fragment_function(Some(&self.frag_func));
        // desc.set_vertex_descriptor(Some(&self.vertex_descriptor));

        // color_attachment_desc.set_blending_enabled(true);
        // color_attachment_desc.set_source_rgb_blend_factor(blend_func.src_rgb);
        // color_attachment_desc.set_source_alpha_blend_factor(blend_func.src_alpha);
        // color_attachment_desc.set_destination_rgb_blend_factor(blend_func.dst_rgb);
        // color_attachment_desc.set_destination_alpha_blend_factor(blend_func.dst_alpha);

        // // self.blend_func = blend_func;
        // // let pipeline_state = self.device.new_render_pipeline_state(&desc).unwrap();
        // // pipeline_state.set_label("pipeline_state");
        // self.pipeline_state = Some(pipeline_state);

        // desc.set_fragment_function(None);
        // color_attachment_desc.set_write_mask(metal::MTLColorWriteMask::empty());
        // let stencil_only_pipeline_state = self.device.new_render_pipeline_state(&desc).unwrap();
        // // stencil_only_pipeline_state.set_label("stencil_only_pipeline_state");
        // self.stencil_only_pipeline_state = Some(stencil_only_pipeline_state);

        // self.pipeline_pixel_format = pixel_format;

        // // the rest of this function is not in metalnvg
        // let clear_rect_pipeline_state = {
        //     let desc2 = metal::RenderPipelineDescriptor::new();
        //     let color_attachment_desc2 = desc2.color_attachments().object_at(0).unwrap();
        //     color_attachment_desc2.set_pixel_format(pixel_format);
        //     // color_attachent_desc.set_pixel_format(metal::MTLPixelFormat::BGRA8Unorm);;
        //     desc2.set_stencil_attachment_pixel_format(metal::MTLPixelFormat::Stencil8);
        //     desc2.set_fragment_function(Some(&self.clear_rect_frag_func));
        //     desc2.set_vertex_function(Some(&self.clear_rect_vert_func));

        //     color_attachment_desc2.set_blending_enabled(true);
        //     color_attachment_desc2.set_source_rgb_blend_factor(blend_func.src_rgb);
        //     color_attachment_desc2.set_source_alpha_blend_factor(blend_func.src_alpha);
        //     color_attachment_desc2.set_destination_rgb_blend_factor(blend_func.dst_rgb);
        //     color_attachment_desc2.set_destination_alpha_blend_factor(blend_func.dst_alpha);

        //     self.device.new_render_pipeline_state(&desc2).unwrap()
        // };

        // // clear_rect_pipeline_state.set_label("clear_rect_pipeline_state");
        // self.clear_rect_pipeline_state = Some(clear_rect_pipeline_state);
    }

    /// done
    pub fn convex_fill(
        &mut self,
        encoder: &metal::RenderCommandEncoderRef,
        images: &ImageStore<MtlTexture>,
        cmd: &Command,
        paint: Params,
    ) {
        let rps = self.current_rps.as_ref().unwrap();
        let pipeline_state = &rps.pipeline_state;

        #[cfg(debug_assertions)]
        encoder.push_debug_group("convex_fill");

        encoder.set_render_pipeline_state(&pipeline_state);
        self.set_uniforms(encoder, images, paint, cmd.image, cmd.alpha_mask);

        //println!("convex_fill start");
        for drawable in &cmd.drawables {
            if let Some((start, count)) = drawable.fill_verts {
                #[cfg(debug_assertions)]
                self.vertex_buffer
                    .add_debug_marker("convex_fill/fill", start as u64..(start + count) as u64);

                //println!("\tconvex_fill/fill: verts #{}: start: {}, count {}", 0, start, count);

                // offset is in bytes
                let offset = self.index_buffer.len();

                let byte_index_buffer_offset = offset * self.index_size;
                // let byte_index_buffer_offset = start * self.index_size;

                // assert!(self.index_buffer.len() == start);
                // triangle_fan_indices_ext(start as u32, count, &mut self.index_buffer);

                // original uses fans so we fake it with indices
                // let indices = triangle_fan_indices_cw(start as u32, count as u32);
                let triangle_fan_index_count = self
                    .index_buffer
                    .extend_with_triange_fan_indices_cw(start as u32, count as u32);
                //println!("\tindex_buffer.extend_from_slice {:?}", indices);
                // self.index_buffer.extend_from_slice(&indices);

                encoder.draw_indexed_primitives(
                    metal::MTLPrimitiveType::Triangle,
                    triangle_fan_index_count as u64, // indices.len() as u64,
                    metal::MTLIndexType::UInt32,
                    self.index_buffer.as_ref(),
                    byte_index_buffer_offset as u64,
                );
            }

            // Draw fringes
            if let Some((start, count)) = drawable.stroke_verts {
                #[cfg(debug_assertions)]
                self.vertex_buffer
                    .add_debug_marker("convex_fill/stroke", start as u64..(start + count) as u64);

                //println!("\tconvex_fill/stroke: verts #{}: start: {}, count {}", 0, start, count);
                encoder.draw_primitives(metal::MTLPrimitiveType::TriangleStrip, start as u64, count as u64)
            }
        }

        //println!("\tconvex_fill/indices {:?}", self.index_buffer);

        #[cfg(debug_assertions)]
        encoder.pop_debug_group();

        //println!("convex_fill end\n");
    }

    /// done
    pub fn concave_fill(
        &mut self,
        encoder: &metal::RenderCommandEncoderRef,
        images: &ImageStore<MtlTexture>,
        cmd: &Command,
        stencil_paint: Params,
        fill_paint: Params,
    ) {
        #[cfg(debug_assertions)]
        encoder.push_debug_group("concave_fill");

        let rps = self.current_rps.as_ref().unwrap();
        let pipeline_state = &rps.pipeline_state;
        let stencil_only_pipeline_state = &rps.stencil_only_pipeline_state;

        encoder.set_cull_mode(metal::MTLCullMode::None);
        encoder.set_depth_stencil_state(&self.fill_shape_stencil_state);
        encoder.set_render_pipeline_state(stencil_only_pipeline_state);

        // todo metal nanovg doesn't have this but gpucanvas does
        self.set_uniforms(encoder, images, stencil_paint, None, None);

        for drawable in &cmd.drawables {
            if let Some((start, count)) = drawable.fill_verts {
                //println!("concave_fill/fill verts #{}: start: {}, count {}", 0, start, count);
                let offset = self.index_buffer.len();
                let byte_index_buffer_offset = offset * self.index_size;
                // let byte_index_buffer_offset = start * self.index_size;
                // assert!(self.index_buffer.len() == start);
                // let indices = triangle_fan_indices_cw(start as u32, count as u32);
                //println!("\tindex_buffer.extend_from_slice {:?}", indices);
                // self.index_buffer.extend_from_slice(&indices);
                let triangle_fan_index_count = self
                    .index_buffer
                    .extend_with_triange_fan_indices_cw(start as u32, count as u32);
                // original uses fans
                encoder.draw_indexed_primitives(
                    metal::MTLPrimitiveType::Triangle,
                    triangle_fan_index_count as u64, // indices.len() as u64,
                    metal::MTLIndexType::UInt32,
                    self.index_buffer.as_ref(),
                    byte_index_buffer_offset as u64,
                );
            }
        }
        // Restores states.
        encoder.set_cull_mode(metal::MTLCullMode::Back);
        encoder.set_render_pipeline_state(pipeline_state);

        // Draws anti-aliased fragments.
        self.set_uniforms(encoder, images, fill_paint, cmd.image, cmd.alpha_mask);
        if self.antialias {
            match cmd.fill_rule {
                FillRule::NonZero => {} //gl::StencilFunc(gl::EQUAL, 0x0, 0xff),
                FillRule::EvenOdd => {} // gl::StencilFunc(gl::EQUAL, 0x0, 0x1),
            }
            encoder.set_depth_stencil_state(&self.fill_anti_alias_stencil_state);

            for drawable in &cmd.drawables {
                if let Some((start, count)) = drawable.stroke_verts {
                    //println!("concave_fill/stroke verts #{}: start: {}, count {}", 0, start, count);
                    encoder.draw_primitives(metal::MTLPrimitiveType::TriangleStrip, start as u64, count as u64);
                }
            }
        }

        // Draws fill.

        match cmd.fill_rule {
            FillRule::NonZero => {} //gl::StencilFunc(gl::NOTEQUAL, 0x0, 0xff),
            FillRule::EvenOdd => {} // gl::StencilFunc(gl::NOTEQUAL, 0x0, 0x1),
        }
        encoder.set_depth_stencil_state(&self.fill_stencil_state);
        if let Some((start, count)) = cmd.triangles_verts {
            //println!("concave_fill/triangles verts #{}: start: {}, count {}", 0, start, count);
            encoder.draw_primitives(metal::MTLPrimitiveType::TriangleStrip, start as u64, count as u64);
        }
        encoder.set_depth_stencil_state(&self.default_stencil_state);

        #[cfg(debug_assertions)]
        encoder.pop_debug_group();
    }

    /// done
    pub fn stroke(
        &mut self,
        encoder: &metal::RenderCommandEncoderRef,
        images: &ImageStore<MtlTexture>,
        cmd: &Command,
        paint: Params,
    ) {
        #[cfg(debug_assertions)]
        encoder.push_debug_group("stroke");

        self.set_uniforms(encoder, images, paint, cmd.image, cmd.alpha_mask);
        for drawable in &cmd.drawables {
            if let Some((start, count)) = drawable.stroke_verts {
                encoder.draw_primitives(metal::MTLPrimitiveType::TriangleStrip, start as u64, count as u64)
            }
        }

        #[cfg(debug_assertions)]
        encoder.pop_debug_group();
    }

    /// done
    pub fn stencil_stroke(
        &mut self,
        encoder: &metal::RenderCommandEncoderRef,
        images: &ImageStore<MtlTexture>,
        cmd: &Command,
        paint1: Params,
        paint2: Params,
    ) {
        #[cfg(debug_assertions)]
        encoder.push_debug_group("stencil_stroke");

        let rps = self.current_rps.as_ref().unwrap();
        let pipeline_state = &rps.pipeline_state;
        let stencil_only_pipeline_state = &rps.stencil_only_pipeline_state;

        // Fills the stroke base without overlap.
        self.set_uniforms(encoder, images, paint2, cmd.image, cmd.alpha_mask);
        encoder.set_depth_stencil_state(&self.stroke_shape_stencil_state);
        encoder.set_render_pipeline_state(pipeline_state);

        for drawable in &cmd.drawables {
            if let Some((start, count)) = drawable.stroke_verts {
                encoder.draw_primitives(metal::MTLPrimitiveType::TriangleStrip, start as u64, count as u64)
            }
        }

        // Draw anti-aliased pixels.
        self.set_uniforms(encoder, images, paint1, cmd.image, cmd.alpha_mask);
        encoder.set_depth_stencil_state(&self.stroke_anti_alias_stencil_state);

        for drawable in &cmd.drawables {
            if let Some((start, count)) = drawable.stroke_verts {
                // unsafe { gl::DrawArrays(gl::TRIANGLE_STRIP, start as i32, count as i32); }
                encoder.draw_primitives(metal::MTLPrimitiveType::TriangleStrip, start as u64, count as u64);
            }
        }

        // Clears stencil buffer.
        encoder.set_depth_stencil_state(&self.stroke_clear_stencil_state);
        encoder.set_render_pipeline_state(&stencil_only_pipeline_state);

        for drawable in &cmd.drawables {
            if let Some((start, count)) = drawable.stroke_verts {
                encoder.draw_primitives(metal::MTLPrimitiveType::TriangleStrip, start as u64, count as u64);
            }
        }
        encoder.set_depth_stencil_state(&self.default_stencil_state);

        #[cfg(debug_assertions)]
        encoder.pop_debug_group();
    }

    /// done
    pub fn triangles(
        &mut self,
        encoder: &metal::RenderCommandEncoderRef,
        images: &ImageStore<MtlTexture>,
        cmd: &Command,
        paint: Params,
    ) {
        #[cfg(debug_assertions)]
        encoder.push_debug_group("triangles");

        let rps = self.current_rps.as_ref().unwrap();
        let pipeline_state = &rps.pipeline_state;

        self.set_uniforms(encoder, images, paint, cmd.image, cmd.alpha_mask);
        encoder.set_render_pipeline_state(&pipeline_state);
        if let Some((start, count)) = cmd.triangles_verts {
            encoder.draw_primitives(metal::MTLPrimitiveType::Triangle, start as u64, count as u64);
        }

        #[cfg(debug_assertions)]
        encoder.pop_debug_group();
    }

    fn set_uniforms(
        &self,
        encoder: &metal::RenderCommandEncoderRef,
        images: &ImageStore<MtlTexture>,
        paint: Params,
        image_tex: Option<ImageId>,
        alpha_tex: Option<ImageId>,
    ) {
        // println!("-------");
        // println!("start: set_uniforms {:?}", image_tex);
        // println!("uniforms render_target {:?}", self.render_target);
        let tex = if let Some(id) = image_tex {
            // if self.render_target == RenderTarget::Image(id) {
            // println!("render_target == image({:?}), setting pseudotexture", id);
            // &self.pseudo_texture
            // } else {
            // println!("render_target != image, setting id {:?}", id);
            images.get(id).unwrap()
        // }
        } else {
            // println!("image_tex == None, setting pseudo texture");
            &self.pseudo_texture
        };

        encoder.set_fragment_texture(0, Some(&tex.tex()));
        encoder.set_fragment_sampler_state(0, Some(&tex.sampler()));
        encoder.set_fragment_value(0, &paint);
        // encoder.set_

        let mut alpha = false;
        let alpha_tex = if let Some(id) = alpha_tex {
            alpha = true;
            // if self.render_target == RenderTarget::Image(id) {
            //     &self.pseudo_texture
            // } else {
            images.get(id).unwrap()
        // }
        } else {
            &self.pseudo_texture
        };

        // if alpha {
        //     encoder.push_debug_group("alpha_tex");
        // }

        encoder.set_fragment_texture(1, Some(&alpha_tex.tex()));
        encoder.set_fragment_sampler_state(1, Some(&alpha_tex.sampler()));
        // println!("end: set_uniforms {:?}", image_tex);
        // if alpha {
        // encoder.pop_debug_group();
        // }
    }

    // from warrenmoore
    // Well, as I think we discussed previously, scissor state doesn’t affect clear load actions in Metal, but you can simulate this by drawing a rect with a solid color with depth read disabled and depth write enabled and forcing the depth to the clear depth value (assuming you’re using a depth buffer)
    // Looks like in this case the depth buffer is irrelevant. Stencil buffer contents can be cleared similarly to the depth buffer, though

    // mnvgclearwithcolor
    pub fn clear_rect(
        &mut self,
        encoder: &metal::RenderCommandEncoderRef,
        _images: &ImageStore<MtlTexture>,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        color: Color,
    ) {
        #[cfg(debug_assertions)]
        encoder.push_debug_group("clear_rect");

        let rps = self.current_rps.as_ref().unwrap();
        let pipeline_state = &rps.pipeline_state;
        let clear_rect_pipeline_state = &rps.clear_rect_pipeline_state;

        // let view_size = *self.view_size_buffer;

        // let rect = Rect {
        //     x: x as _,
        //     // todo: this is a huge hack around the fact that i'm not sure
        //     // how to properly flip the y coordinate
        //     // it doesn't matter for single color rectangles
        //     y: (view_size.h as u32 - y - height) as _,
        //     // y: y as _,
        //     w: width as _,
        //     h: height as _,
        // };
        // let ndc_rect = rect.as_ndc((view_size.w, view_size.h));

        // let ndc_rect1 = Rect {
        //     x: 2.0 * (x as f32) / view_size.w - 1.0,
        //     // y: (1.0 - 2.0 * (y as f32) / view_size.h),
        //     y: 2.0 * (y as f32) / view_size.h - 1.0,
        //     w: (width as f32) / view_size.w,
        //     h: -(height as f32) / view_size.h,
        // };
        // println!("clear_rect {:?}", ndc_rect1);
        let ndc_rect = Rect {
            x: -1.0,
            y: -1.0,
            w: 2.0,
            h: 2.0,
        };
        let clear_rect = ClearRect { rect: ndc_rect, color };

        encoder.set_render_pipeline_state(clear_rect_pipeline_state);
        encoder.set_vertex_value(0, &clear_rect);
        encoder.set_scissor_rect(metal::MTLScissorRect {
            x: x as _,
            y: y as _,
            width: width as _,
            height: height as _,
        });

        encoder.draw_primitives_instanced(metal::MTLPrimitiveType::TriangleStrip, 0, 4, 1);

        // reset state
        // let size = *self.view_size_buffer;
        let size = self.view_size;
        encoder.set_scissor_rect(metal::MTLScissorRect {
            x: 0,
            y: 0,
            width: size.w as _,
            height: size.h as _,
        });

        // reset buffers for the other commands
        encoder.set_render_pipeline_state(&pipeline_state);
        encoder.set_vertex_buffer(0, Some(self.vertex_buffer.as_ref()), 0);
        // encoder.set_vertex_buffer(1, Some(self.view_size_buffer.as_ref()), 0);
        encoder.set_vertex_value(1, &size);

        #[cfg(debug_assertions)]
        encoder.pop_debug_group();
    }

    pub fn set_target(&mut self, images: &ImageStore<MtlTexture>, target: RenderTarget) {
        if self.render_target == target {
            assert!(false);
            return;
        }

        // let prev_size = self.view_size;

        let size = match target {
            RenderTarget::Screen => self.layer.drawable_size().into(),
            RenderTarget::Image(id) => {
                let texture = images.get(id).unwrap();
                texture.size()
            }
            RenderTarget::None => unimplemented!("rendertarget cannot be none"),
        };

        // println!(
        //     "frame: {:?}, setting target from {:?}({:?}) to {:?}({:?})",
        //     self.frame, self.render_target, prev_size, target, size
        // );

        self.render_target = target;

        self.view_size = size;
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
}

// fn to_ndc(position: (f32, f32), drawable_size: (f32, f32)) -> (f32, f32) {
//     let x_scale = 2.0 / drawable_size.0;
//     let y_scale = 2.0 / drawable_size.1;
//     let x_bias = -1.0;
//     let y_bias = -1.0;

//     let x = position.0 * x_scale + x_bias;
//     let y = position.1 * y_scale + y_bias;
//     (x, y)
// }

// impl Rect {
//     pub fn as_ndc_flipped(&self, screen_size: (f32, f32)) -> Self {
//         let src = (self.x, self.y);
//         let dst = (self.x + self.w, self.y + self.h);

//         let ndc_src = to_ndc(src, screen_size);
//         let ndc_dst = to_ndc(dst, screen_size);

//         let x = ndc_src.0;
//         let y = ndc_src.1;
//         let w = ndc_dst.0 - ndc_src.0;
//         let h = ndc_dst.1 - ndc_src.1;
//         Self { x, y, w, h }
//     }
//     pub fn as_ndc(&self, screen_size: (f32, f32)) -> Self {
//         let src = (self.x, self.y);
//         // let dst = (self.x + self.w, self.y + self.h);

//         let ndc_src = to_ndc(src, screen_size);
//         // let ndc_dst = to_ndc(dst, screen_size);

//         let x = ndc_src.0;
//         let y = ndc_src.1;
//         let w = self.w / screen_size.0 * 2.0;
//         let h = self.h / screen_size.1 * 2.0;
//         Self { x, y, w, h }
//     }
// }

impl From<Color> for metal::MTLClearColor {
    fn from(v: Color) -> Self {
        Self::new(v.r.into(), v.g.into(), v.b.into(), v.a.into())
    }
}

static mut SHOULD_RENDER: bool = true;

fn lock() {
    unsafe {
        SHOULD_RENDER = false;
    }
}

fn unlock() {
    unsafe {
        SHOULD_RENDER = true;
    }
}

fn should_render() -> bool {
    unsafe { SHOULD_RENDER }
}

fn new_render_command_encoder<'a>(
    target_texture: &metal::TextureRef,
    command_buffer: &'a metal::CommandBufferRef,
    clear_color: Color,
    stencil_texture: &mut StencilTexture,
    // view_size: Size,
    vertex_buffer: &GPUVec<Vertex>,
    // view_size_buffer: &GPUVar<Size>,
    view_size: Size,
    // index_buffer: &IndexBuffer,
    // uniform_buffer: &GPUVec<Params>,
    // clear_buffer_on_flush: bool,
) -> &'a metal::RenderCommandEncoderRef {
    if true {
        let load_action =
        // if clear_buffer_on_flush {
            metal::MTLLoadAction::Load;
        // } else {
        // metal::MTLLoadAction::Clear;
        // };
        let desc = metal::RenderPassDescriptor::new();

        stencil_texture.resize(view_size);
        // let view_size = **view_size_buffer;

        let color_attachment = desc.color_attachments().object_at(0).unwrap();
        color_attachment.set_clear_color(clear_color.into());
        color_attachment.set_load_action(load_action);
        color_attachment.set_store_action(metal::MTLStoreAction::Store);
        color_attachment.set_texture(Some(&target_texture));
        // added

        let stencil_attachment = desc.stencil_attachment().unwrap();
        stencil_attachment.set_clear_stencil(0);
        stencil_attachment.set_load_action(metal::MTLLoadAction::Clear);
        stencil_attachment.set_store_action(metal::MTLStoreAction::DontCare);
        stencil_attachment.set_texture(Some(&stencil_texture.tex()));

        let mut encoder = command_buffer.new_render_command_encoder(&desc);

        encoder.set_cull_mode(metal::MTLCullMode::Back);
        encoder.set_front_facing_winding(metal::MTLWinding::CounterClockwise);
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
        // encoder.set_vertex_buffer(1, Some(view_size_buffer.as_ref()), 0);
        encoder.set_vertex_value(1, &view_size);
        // encoder.set_fragment_buffer(0, Some(uniform_buffer.as_ref()), 0);

        encoder
    } else {
        todo!()
        //     let desc = metal::RenderPassDescriptor::new();
        //     let color_attachment = desc.color_attachments().object_at(0).unwrap();

        //     color_attachment.set_texture(Some(color_texture));
        //     color_attachment.set_load_action(metal::MTLLoadAction::Clear);
        //     color_attachment.set_clear_color(clear_color.into());
        //     color_attachment.set_store_action(metal::MTLStoreAction::Store);
        //     command_buffer.new_render_command_encoder(&desc)
    }
}

impl crate::renderer::BufferCache for MtlBuffersCache {}

impl Renderer for Mtl {
    type Image = MtlTexture;
    // type BufferCache = MtlBuffersCache;
    type BufferCache = crate::renderer::VoidCache;

    fn alloc_buffer_cache(&self) -> Self::BufferCache {
        // Self::BufferCache::new(&self.device, self.multiple_buffering)
        Self::BufferCache::new()
    }

    fn view_size(&self) -> Size {
        // *self.view_size_buffer
        self.view_size
        // match self.render_target {
        //     RenderTarget::Screen => {
        //         self.layer.drawable_size().into()
        //     }
        //     RenderTarget::Image(id) => {
        //         todo!()
        //     }
        // }
    }

    fn set_size(&mut self, width: u32, height: u32, dpi: f32) {
        let size = Size::new(width as f32, height as f32);
        // self.screen_view = [width as _, height as _];
        // *self.view_size_buffer = size;
        self.view_size = size;
    }

    fn start_capture(&self) {
        let shared = metal::CaptureManager::shared();
        shared.start_capture_with_command_queue(&self.command_queue);
    }

    fn stop_capture(&self) {
        let shared = metal::CaptureManager::shared();
        shared.stop_capture();
    }

    fn label(&self, images: &ImageStore<Self::Image>, id: ImageId) -> String {
        let img = images.get(id).unwrap();
        img.label().to_owned()
    }

    fn set_label(&self, images: &ImageStore<Self::Image>, id: ImageId, label: &str) {
        let img = images.get(id).unwrap();
        img.set_label(label)
    }

    // called flush in ollix and nvg
    fn render(
        &mut self,
        images: &ImageStore<Self::Image>,
        cache: &mut Self::BufferCache,
        verts: &[Vertex],
        commands: &[Command],
    ) {
        //println!("verts len {:?}", verts.len());
        //// println!("index_buffer.byte_len {}", self.index_buffer.byte_len());
        //// println!("index_buffer.byte_capacity {}", self.index_buffer.byte_capacity());
        if !should_render() {
            // println!("dropping frame with target ");
            return;
        }
        lock();

        #[derive(Copy, Clone, Default, Debug)]
        struct Counters {
            convex_fill: usize,
            concave_fill: usize,
            stroke: usize,
            stencil_stroke: usize,
            triangles: usize,
            clear_rect: usize,
            set_render_target: usize,
        }

        let mut counters: Counters = Default::default();

        // let lens = PathsLength::new(commands);
        // let max_verts = lens.vertex_count + lens.triangle_count;

        #[cfg(debug_assertions)]
        self.vertex_buffer.remove_all_debug_markers();

        self.vertex_buffer.clear();
        // self.index_buffer.resize(max_verts);
        // self.vertex_buffer.resize(verts.len());
        self.vertex_buffer.extend_from_slice(verts);

        // build indices
        self.index_buffer.clear();
        self.index_buffer.resize(3 * verts.len());
        //// println!("reserving {}", 3 * verts.len());
        // temporary to ensure that the index_buffer is does not
        // change the inner allocation
        // the reserve should allocate enough
        let vertex_buffer_hash = self.vertex_buffer.ptr_hash();
        let index_buffer_hash = self.index_buffer.ptr_hash();

        // let e = BUFFER_CACHE.acquire();
        // let (buffers_index, buffers) = self.buffers_cache.acquire();

        // let mut stroke_vert_offset = max_verts - lens.stroke_count;

        // for cmd in commands {
        //     for drawable in &cmd.drawables {
        //         if let Some((start, count)) = drawable.fill_verts {
        //             if count > 2 {
        //                 let mut hub_offset = self.vertex_buffer.len() as u32;
        //                 // hub_offset += 1;
        //                 // self.vertex_buffer.splice_slow(..2, verts[start..start+count].iter().cloned());
        //                 self.vertex_buffer.extend_from_slice(&verts[start..start+count]);
        //                 for index in 2..count {
        //                     self.index_buffer.extend_from_slice(&[hub_offset,
        //                                                 (start + index) as u32,
        //                                                 (start + index + 1) as u32]);
        //                 }
        //             }
        //         }

        //         if let Some((start, count)) = drawable.stroke_verts {
        //             if count > 0 {
        //                 self.vertex_buffer.extend_from_slice(&verts[start..start+count]);
        //                 // self.vertex_buffer.splice_slow(stroke_vert_offset..stroke_vert_offset+count,
        //                 //     verts[start..start+count].iter().cloned());
        //                 //     stroke_vert_offset += count;
        //                 // unsafe {
        //                     // std::ptr::copy(
        //                     //     &verts[start..start+count],
        //                     //     self.vertex_buffer.as_mut_ptr() as _,
        //                     //     0
        //                     // );

        //                 // }

        //                 // ;
        //                 // vertex_count += count + 2;
        //                 // stroke_count += count;
        //             }
        //         }
        //     }

        //     // if let Some((start, count)) = cmd.triangles_verts {
        //     //     // triangle_count += count;
        //     // }
        // }

        let clear_color: Color = self.clear_color;
        //// println!("clear_color: {:?}", clear_color);

        let command_buffer = self.command_queue.new_command_buffer().to_owned();
        command_buffer.enqueue();
        let block = block::ConcreteBlock::new(move |buffer: &metal::CommandBufferRef| {
            //     // println!("{}", buffer.label());
            // self.vertex_buffer.clear();

            unlock();
        })
        .copy();
        command_buffer.add_completed_handler(&block);
        let mut drawable: Option<metal::MetalDrawable> = None;

        let mut target_texture = match self.render_target {
            RenderTarget::Screen => {
                // println!("render target: screen");
                let d = self.layer.next_drawable().unwrap().to_owned();
                let tex = d.texture().to_owned();
                drawable = Some(d);
                tex
            }
            RenderTarget::Image(id) => {
                // println!("render target: image: {:?}", id);
                images.get(id).unwrap().tex().to_owned()
            }
            RenderTarget::None => unimplemented!("rendertarget cannot be none"),
        };

        // this is needed for screenshotting
        // self.last_rendered_texture = Some(color_texture.to_owned());

        let size = Size::new(target_texture.width() as _, target_texture.height() as _);
        // println!("target_texture size: {:?}", size);
        // println!("pre stencil texture size: {:?}", self.stencil_texture.size());

        // self.temp_texture.resize(size);
        // println!("pre stencil texture size: {:?}", self.stencil_texture.size());
        // assert_eq!(size, *self.view_size_buffer);
        let mut encoder = new_render_command_encoder(
            // &self.temp_texture.tex,
            &target_texture,
            &command_buffer,
            clear_color,
            &mut self.stencil_texture,
            &self.vertex_buffer,
            // &self.view_size_buffer,
            self.view_size
            // &self.uniform_buffer,
            // self.clear_buffer_on_flush,
        );

        encoder.push_debug_group(&format!("frame: {:?}", self.frame));

        let mut pixel_format = target_texture.pixel_format();
        encoder.push_debug_group(&format!("target: {:?}", self.render_target));
        // match self.render_target {
        //     RenderTarget::Screen => {
        //         encoder.push_debug_group("rendering to screen");
        //     }
        //     RenderTarget::Image(id) => {
        //         encoder.push_debug_group(&format!("rendering to image: {:?}", id));
        //     }
        // }
        // self.stencil_texture.resize();
        // self.clear_buffer_on_flush = false;p

        // pub struct RenderContext {}

        // let mut switches = vec![];

        // for (i, cd) in commands.iter().enumerate() {

        // }

        // fn dump_command_type(cmd: &Command) -> &str {
        //     match cmd.cmd_type {
        //         CommandType::ConvexFill { .. } => "convex_fill",
        //         CommandType::ConcaveFill { .. } => "concave_fill",
        //         CommandType::Stroke { .. } => "stroke",
        //         CommandType::StencilStroke { .. } => "stencil_stroke",
        //         CommandType::Triangles { .. } => "triangles",
        //         CommandType::ClearRect { .. } => "clear_rect",
        //         CommandType::SetRenderTarget { .. } => "set_render_target",
        //     }
        // }
        // println!("loop start");
        let mut target_set = 0;
        for cmd in commands {
            // println!("command_type: {:?}", dump_command_type(cmd));
            self.set_composite_operation(cmd.composite_operation, pixel_format);

            match cmd.cmd_type {
                CommandType::ConvexFill { params } => {
                    //counters.convex_fill += 1;
                    self.convex_fill(&encoder, images, cmd, params)
                }
                CommandType::ConcaveFill {
                    stencil_params,
                    fill_params,
                } => {
                    //counters.concave_fill += 1;
                    self.concave_fill(&encoder, images, cmd, stencil_params, fill_params)
                }
                CommandType::Stroke { params } => {
                    //counters.stroke += 1;
                    self.stroke(&encoder, images, cmd, params)
                }
                CommandType::StencilStroke { params1, params2 } => {
                    //counters.stencil_stroke += 1;
                    self.stencil_stroke(&encoder, images, cmd, params1, params2)
                }
                CommandType::Triangles { params } => {
                    //counters.triangles += 1;
                    self.triangles(&encoder, images, cmd, params)
                }
                CommandType::ClearRect {
                    x,
                    y,
                    width,
                    height,
                    color,
                } => {
                    //counters.clear_rect += 1;
                    self.clear_rect(&encoder, images, x, y, width, height, color);
                }
                CommandType::SetRenderTarget(target) => {
                    if self.render_target == target {
                        // println!("skipping target setting");
                        continue;
                    }
                    encoder.pop_debug_group();

                    target_set += 1;
                    //counters.set_render_target += 1;
                    // println!("---------switching from {:?} to {:?}", self.render_target, target);

                    encoder.end_encoding();
                    self.set_target(images, target);

                    // if let Some(drawable) = drawable.as_ref() {
                    //     command_buffer.present_drawable(&drawable);
                    // }

                    let size;
                    target_texture = match self.render_target {
                        RenderTarget::Screen => {
                            // println!("render target: screen");
                            let d = self.layer.next_drawable().unwrap().to_owned();
                            let tex = d.texture().to_owned();
                            drawable = Some(d);
                            size = self.layer.drawable_size().into();
                            tex
                        }
                        RenderTarget::Image(id) => {
                            // println!("render target: image: {:?}", id);
                            let tex = images.get(id).unwrap().tex().to_owned();
                            size = tex.size();
                            tex
                        }
                        RenderTarget::None => unimplemented!("rendertarget cannot be none"),
                    };
                    pixel_format = target_texture.pixel_format();
                    // println!("size0: {:?}, size1: {:?}", size, *self.view_size_buffer);
                    // assert!(size == *self.view_size_buffer);
                    assert!(size == self.view_size);

                    encoder = new_render_command_encoder(
                        &target_texture,
                        &command_buffer,
                        clear_color,
                        &mut self.stencil_texture,
                        &self.vertex_buffer,
                        // &self.view_size_buffer,
                        self.view_size,
                        // &self.uniform_buffer,
                        // self.clear_buffer_on_flush,
                    );
                    encoder.push_debug_group(&format!("target: {:?}", target));
                }
            }
        }
        // println!("loop end");
        // pop the target debug group
        encoder.pop_debug_group();
        // pop the frame debug group
        encoder.pop_debug_group();

        encoder.end_encoding();
        // it appears that having this print statement influences things
        // println!("target_set: {:?}", target_set);

        // println!("pre present");
        if let Some(drawable) = drawable {
            command_buffer.present_drawable(&drawable);
        }

        // println!("target set {}", target_set);
        // println!("pre blit");
        // Makes mnvgReadPixels-like functions (e.g. screenshot) work as expected on Mac.
        #[cfg(target_os = "macos")]
        {
            if self.render_target == RenderTarget::Screen {
                let blit = command_buffer.new_blit_command_encoder();
                blit.synchronize_resource(&target_texture);
                blit.end_encoding();
            }
        }

        // println!("post blit");
        command_buffer.commit();

        // println!("post commit");
        // if self.frame == 1 {
        //     color_texture.save_to("/Users/adamnemecek/Code/ngrid/main/vendor/ngrid10deps/gpucanvas/out.png");
        // }
        self.frame += 1;

        assert!(vertex_buffer_hash == self.vertex_buffer.ptr_hash());
        assert!(index_buffer_hash == self.index_buffer.ptr_hash());

        // command_buffer.wait_until_scheduled();
        // println!("counters {:?}", counters);

        // if !self.layer.presents_with_transaction() {
        //     command_buffer.present_drawable(&drawable);
        // }

        // if self.layer.presents_with_transaction() {
        //     command_buffer.wait_until_scheduled();
        //     // drawable.present();
        // }
    }

    fn alloc_image(&mut self, info: ImageInfo) -> Result<Self::Image, ErrorKind> {
        let mut img = Self::Image::new(&self.device, &self.command_queue, info);
        img.as_mut().unwrap().set_clear_color(rgb::RGBA8 {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        });
        img
    }

    fn update_image(
        &mut self,
        image: &mut Self::Image,
        data: ImageSource,
        x: usize,
        y: usize,
    ) -> Result<(), ErrorKind> {
        image.update(data, x, y)
    }

    fn delete_image(&mut self, image: Self::Image) {
        image.delete();
    }

    // fn set_target(&mut self, images: &ImageStore<MtlTexture>, target: RenderTarget) {
    //     self.render_target = target;
    //     todo!();
    // }

    fn flip_y() -> bool {
        true
    }

    fn flip_uv() -> bool {
        true
    }

    fn screenshot(&mut self, images: &ImageStore<Self::Image>) -> Result<ImgVec<RGBA8>, ErrorKind> {
        println!("screenshot: {:?}", self.render_target);

        let texture = match self.render_target {
            RenderTarget::Screen => self.layer.next_drawable().map(|x| x.texture()),
            RenderTarget::Image(id) => images.get(id).map(|x| x.tex()),
            RenderTarget::None => unimplemented!("rendertarget cannot be none"),
        }
        .unwrap();
        // let texture = self.last_rendered_texture.as_ref().unwrap();

        // todo!()
        // look at headless renderer in metal-rs
        // let size = *self.view_size_buffer;
        let width = texture.width();
        let height = texture.height();
        let w = width as u64;
        let h = height as u64;

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

        texture.get_bytes(
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

        Ok(buffer)
    }
}

// pub struct SolidTexture {
//     w: usize,
//     h: usize,
//     color: Color,
//     inner: Vec<Color>
// }

// impl SolidTexture {
//     pub fn new(w: usize, h: usize, color: Color) -> Self {
//         Self {
//             w,
//             h,
//             color,
//             inner: vec![color; w * h]
//         }
//     }

//     pub fn update(&mut self, w: usize, h: usize, color: Color) {
//         if self.w * self.h < w * h {
//             self.inner = vec![color; w * h]
//         }
//         self.w = w;
//         self.h = h;
//     }
// }
