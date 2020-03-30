
// use std::ptr;
// use std::mem;
// use std::ops::DerefMut;
// use std::ffi::{CString, CStr, c_void};

use image::DynamicImage;

use crate::{
    Color,
    Result,
    ImageStore,
    FillRule,
    CompositeOperationState,
    BlendFactor,
    renderer::{Vertex, ImageId}
};

use super::{
    Params,
    Renderer,
    Command,
    CommandType,
    ImageFlags
};

// mod program;
// use program::{
//     Shader,
//     Program
// };

// mod texture;
// use texture::Texture;
mod mtl_texture;
use mtl_texture::MtlTexture;

// mod uniform_array;
// use uniform_array::UniformArray;

// #[allow(clippy::all)]
// mod gl {
//     include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
// }

// use gl::types::*;

pub struct Mtl {
    debug: bool,
    antialias: bool,
    // is_opengles: bool,
    view: [f32; 2],

    device: metal::Device,
    // library: metal::Library,
    command_queue: metal::CommandQueue,
    frag: metal::Function,
    vert: metal::Function,
    render_encoder: Option<metal::RenderCommandEncoder>

    // program: Program,
    // vert_arr: GLuint,
    // vert_buff: GLuint,
}

impl Mtl {
    // pub fn new<F>(load_fn: F) -> Result<Self>  where F : Copy {
    pub fn new() -> Result<Self> {
        let device = metal::Device::system_default().unwrap();
        let library = device.new_library_with_file("sha ders.metallib").expect("library not found");
        let command_queue = device.new_command_queue();
    
        let debug = true;
        let antialias = true;

        // gl::load_with(load_fn);

        // let shader_defs = if antialias { "#define EDGE_AA 1" } else { "" };
        // let vert_shader_src = format!("#version 100\n{}\n{}", shader_defs, include_str!("main-vs.glsl"));
        // let frag_shader_src = format!("#version 100\n{}\n{}", shader_defs, include_str!("main-fs.glsl"));

        // let vert_shader = Shader::new(&CString::new(vert_shader_src)?, gl::VERTEX_SHADER)?;
        // let frag_shader = Shader::new(&CString::new(frag_shader_src)?, gl::FRAGMENT_SHADER)?;

        // let program = Program::new(&[vert_shader, frag_shader])?;
        let vert = library.get_function("vertexShader", None).expect("vert shader not found");
        // let frag = flags.contains(nvg::ImageFlags::)
        let frag: metal::Function = if antialias {
            library.get_function("fragmentShader", None).expect("frag shader not found")
        } else {
            library.get_function("fragmentShaderAA", None).expect("frag shader not found")
        };

        let mut renderer = Mtl {
            debug: debug,
            antialias: antialias,
            // is_opengles: false,
            view: [0.0, 0.0],
            // program: program,
            
            // vert_arr: 0,
            // vert_buff: 0,
            device,
            command_queue,
            frag,
            vert,
            render_encoder: None
        };

        // unsafe {
        //     let version = CStr::from_ptr(gl::GetString(gl::VERSION) as *mut i8);
        //     opengl.is_opengles = version.to_str().ok().map_or(false, |str| str.starts_with("OpenGL ES"));

        //     gl::GenVertexArrays(1, &mut opengl.vert_arr);
        //     gl::GenBuffers(1, &mut opengl.vert_buff);
        // }

        // Ok(opengl)
        todo!()
    }

//     fn check_error(&self, label: &str) {
//         if !self.debug { return }

//         let err = unsafe { gl::GetError() };

//         if err == gl::NO_ERROR { return; }

//         let message = match err {
//             gl::INVALID_ENUM => "Invalid enum",
//             gl::INVALID_VALUE => "Invalid value",
//             gl::INVALID_OPERATION => "Invalid operation",
//             gl::OUT_OF_MEMORY => "Out of memory",
//             gl::INVALID_FRAMEBUFFER_OPERATION => "Invalid framebuffer operation",
//             _ => "Unknown error"
//         };

//         eprintln!("({}) Error on {} - {}", err, label, message);
//     }

    fn factor(factor: BlendFactor) -> metal::MTLBlendFactor {
        use metal::MTLBlendFactor;

        match factor {
            BlendFactor::Zero => MTLBlendFactor::Zero,
            BlendFactor::One => MTLBlendFactor::One,
            BlendFactor::SrcColor => MTLBlendFactor::SourceColor,
            BlendFactor::OneMinusSrcColor => MTLBlendFactor::OneMinusSourceColor,
            BlendFactor::DstColor => MTLBlendFactor::DestinationColor,
            BlendFactor::OneMinusDstColor => MTLBlendFactor::OneMinusDestinationColor,
            BlendFactor::SrcAlpha => MTLBlendFactor::SourceAlpha,
            BlendFactor::OneMinusSrcAlpha => MTLBlendFactor::OneMinusSourceAlpha,
            BlendFactor::DstAlpha => MTLBlendFactor::DestinationAlpha,
            BlendFactor::OneMinusDstAlpha => MTLBlendFactor::OneMinusDestinationAlpha,
            BlendFactor::SrcAlphaSaturate => MTLBlendFactor::SourceAlphaSaturated,
        }
    }

    fn set_composite_operation(&self, blend_state: CompositeOperationState) {
//         unsafe {
//             gl::BlendFuncSeparate(
//                 Self::gl_factor(blend_state.src_rgb),
//                 Self::gl_factor(blend_state.dst_rgb),
//                 Self::gl_factor(blend_state.src_alpha),
//                 Self::gl_factor(blend_state.dst_alpha)
//             );
//         }
    }

    fn convex_fill(&self, images: &ImageStore<MtlTexture>, cmd: &Command, gpu_paint: Params) {
        self.set_uniforms(images, gpu_paint, cmd.image, cmd.alpha_mask);

        for drawable in &cmd.drawables {
            if let Some((start, count)) = drawable.fill_verts {
                // unsafe { gl::DrawArrays(gl::TRIANGLE_FAN, start as i32, count as i32); }
            }

            if let Some((start, count)) = drawable.stroke_verts {
                // unsafe { gl::DrawArrays(gl::TRIANGLE_STRIP, start as i32, count as i32); }
            }
        }

//         self.check_error("convex_fill");
    }

    fn concave_fill(&self, images: &ImageStore<MtlTexture>, cmd: &Command, stencil_paint: Params, fill_paint: Params) {
//         unsafe {
//             gl::Enable(gl::STENCIL_TEST);
//             gl::StencilMask(0xff);
//             gl::StencilFunc(gl::ALWAYS, 0, 0xff);
//             gl::ColorMask(gl::FALSE, gl::FALSE, gl::FALSE, gl::FALSE);
//             //gl::DepthMask(gl::FALSE);
//         }

        self.set_uniforms(images, stencil_paint, None, None);

//         unsafe {
//             gl::StencilOpSeparate(gl::FRONT, gl::KEEP, gl::KEEP, gl::INCR_WRAP);
//             gl::StencilOpSeparate(gl::BACK, gl::KEEP, gl::KEEP, gl::DECR_WRAP);
//             gl::Disable(gl::CULL_FACE);
//         }

        for drawable in &cmd.drawables {
            if let Some((start, count)) = drawable.fill_verts {
                // unsafe { gl::DrawArrays(gl::TRIANGLE_FAN, start as i32, count as i32); }
            }
        }

//         unsafe {
//             gl::Enable(gl::CULL_FACE);
//             // Draw anti-aliased pixels
//             gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
//             //gl::DepthMask(gl::TRUE);
//         }

//         self.set_uniforms(images, fill_paint, cmd.image, cmd.alpha_mask);

//         if self.antialias {
//             unsafe {
//                 match cmd.fill_rule {
//                     FillRule::NonZero => gl::StencilFunc(gl::EQUAL, 0x0, 0xff),
//                     FillRule::EvenOdd => gl::StencilFunc(gl::EQUAL, 0x0, 0x1)
//                 }

//                 gl::StencilOp(gl::KEEP, gl::KEEP, gl::KEEP);
//             }

//             // draw fringes
//             for drawable in &cmd.drawables {
//                 if let Some((start, count)) = drawable.stroke_verts {
//                     unsafe { gl::DrawArrays(gl::TRIANGLE_STRIP, start as i32, count as i32); }
//                 }
//             }
//         }

//         unsafe {
//             match cmd.fill_rule {
//                 FillRule::NonZero => gl::StencilFunc(gl::NOTEQUAL, 0x0, 0xff),
//                 FillRule::EvenOdd => gl::StencilFunc(gl::NOTEQUAL, 0x0, 0x1)
//             }

//             gl::StencilOp(gl::ZERO, gl::ZERO, gl::ZERO);

//             if let Some((start, count)) = cmd.triangles_verts {
//                 gl::DrawArrays(gl::TRIANGLE_STRIP, start as i32, count as i32);
//             }

//             gl::Disable(gl::STENCIL_TEST);
//         }

//         self.check_error("concave_fill");
    }

    fn stroke(&self, images: &ImageStore<MtlTexture>, cmd: &Command, paint: Params) {
        self.set_uniforms(images, paint, cmd.image, cmd.alpha_mask);

//         for drawable in &cmd.drawables {
//             if let Some((start, count)) = drawable.stroke_verts {
//                 unsafe { gl::DrawArrays(gl::TRIANGLE_STRIP, start as i32, count as i32); }
//             }
//         }

//         self.check_error("stroke");
    }

    fn stencil_stroke(&self, images: &ImageStore<MtlTexture>, cmd: &Command, paint1: Params, paint2: Params) {
//         unsafe {
//             gl::Enable(gl::STENCIL_TEST);
//             gl::StencilMask(0xff);

//             // Fill the stroke base without overlap
//             gl::StencilFunc(gl::EQUAL, 0x0, 0xff);
//             gl::StencilOp(gl::KEEP, gl::KEEP, gl::INCR);
//         }

        self.set_uniforms(images, paint2, cmd.image, cmd.alpha_mask);

        for drawable in &cmd.drawables {
            if let Some((start, count)) = drawable.stroke_verts {
                // unsafe { gl::DrawArrays(gl::TRIANGLE_STRIP, start as i32, count as i32); }
            }
        }

        // Draw anti-aliased pixels.
        self.set_uniforms(images, paint1, cmd.image, cmd.alpha_mask);

//         unsafe {
//             gl::StencilFunc(gl::EQUAL, 0x0, 0xff);
//             gl::StencilOp(gl::KEEP, gl::KEEP, gl::KEEP);
//         }

//         for drawable in &cmd.drawables {
//             if let Some((start, count)) = drawable.stroke_verts {
//                 unsafe { gl::DrawArrays(gl::TRIANGLE_STRIP, start as i32, count as i32); }
//             }
//         }

//         unsafe {
//             // Clear stencil buffer.
//             gl::ColorMask(gl::FALSE, gl::FALSE, gl::FALSE, gl::FALSE);
//             gl::StencilFunc(gl::ALWAYS, 0x0, 0xff);
//             gl::StencilOp(gl::ZERO, gl::ZERO, gl::ZERO);
//         }

//         for drawable in &cmd.drawables {
//             if let Some((start, count)) = drawable.stroke_verts {
//                 unsafe { gl::DrawArrays(gl::TRIANGLE_STRIP, start as i32, count as i32); }
//             }
//         }

//         unsafe {
//             gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
//             gl::Disable(gl::STENCIL_TEST);
//         }

//         self.check_error("stencil_stroke");
    }

    fn triangles(&self, images: &ImageStore<MtlTexture>, cmd: &Command, paint: Params) {
        self.set_uniforms(images, paint, cmd.image, cmd.alpha_mask);

        if let Some((start, count)) = cmd.triangles_verts {
            // unsafe { gl::DrawArrays(gl::TRIANGLES, start as i32, count as i32); }
            // self.render_encoder
        }

//         self.check_error("triangles");
    }

    fn set_uniforms(&self, images: &ImageStore<MtlTexture>, paint: Params, image_tex: Option<ImageId>, alpha_tex: Option<ImageId>) {
//         let arr = UniformArray::from(paint);
//         self.program.set_config(UniformArray::size() as i32, arr.as_ptr());
//         self.check_error("set_uniforms uniforms");

        let tex = image_tex.and_then(|id| images.get(id)).map_or(0, |tex| tex.id());

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

    fn clear_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: Color) {
//         unsafe {
//             gl::Enable(gl::SCISSOR_TEST);
//             gl::Scissor(x as i32, self.view[1] as i32 - (height as i32 + y as i32), width as i32, height as i32);
//             gl::ClearColor(color.r, color.g, color.b, color.a);
//             gl::Clear(gl::COLOR_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
//             gl::Disable(gl::SCISSOR_TEST);
//         }
    }
}

impl Renderer for Mtl {
    type Image = MtlTexture;

    fn set_size(&mut self, width: u32, height: u32, _dpi: f32) {
        self.view[0] = width as f32;
        self.view[1] = height as f32;

//         unsafe {
//             gl::Viewport(0, 0, width as i32, height as i32);
//         }
    }

    // called flush in ollix and nvg
    fn render(&mut self, images: &ImageStore<Self::Image>, verts: &[Vertex], commands: &[Command]) {
        let command_buffer = self.command_queue.new_command_buffer();
        let render_pass_descriptor = metal::RenderPassDescriptor::new();
        let render_encoder = command_buffer.new_render_command_encoder(
            &render_pass_descriptor
        );
//         self.program.bind();

//         unsafe {
//             gl::Enable(gl::CULL_FACE);

//             gl::CullFace(gl::BACK);
//             gl::FrontFace(gl::CCW);
//             gl::Enable(gl::BLEND);
//             gl::Disable(gl::DEPTH_TEST);
//             gl::Disable(gl::SCISSOR_TEST);
//             gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
//             gl::StencilMask(0xffff_ffff);
//             gl::StencilOp(gl::KEEP, gl::KEEP, gl::KEEP);
//             gl::StencilFunc(gl::ALWAYS, 0, 0xffff_ffff);
//             gl::ActiveTexture(gl::TEXTURE0);
//             gl::BindTexture(gl::TEXTURE_2D, 0);

//             gl::BindVertexArray(self.vert_arr);

//             let vertex_size = mem::size_of::<Vertex>();

//             gl::BindBuffer(gl::ARRAY_BUFFER, self.vert_buff);
//             let size = verts.len() * vertex_size;
//             gl::BufferData(gl::ARRAY_BUFFER, size as isize, verts.as_ptr() as *const GLvoid, gl::STREAM_DRAW);

//             gl::EnableVertexAttribArray(0);
//             gl::EnableVertexAttribArray(1);

//             gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, vertex_size as i32, ptr::null::<c_void>());
//             gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, vertex_size as i32, (2 * mem::size_of::<f32>()) as *const c_void);
//         }

//         // Bind the two uniform samplers to texture units
//         self.program.set_tex(0);
//         self.program.set_masktex(1);
//         // Set uniforms
//         self.program.set_view(self.view);

//         self.check_error("render prepare");

        for cmd in commands {
            self.set_composite_operation(cmd.composite_operation);

            match cmd.cmd_type {
                CommandType::ConvexFill { params } => self.convex_fill(images, cmd, params),
                CommandType::ConcaveFill { stencil_params, fill_params } => self.concave_fill(images, cmd, stencil_params, fill_params),
                CommandType::Stroke { params } => self.stroke(images, cmd, params),
                CommandType::StencilStroke { params1, params2 } => self.stencil_stroke(images, cmd, params1, params2),
                CommandType::Triangles { params } => self.triangles(images, cmd, params),
                CommandType::ClearRect { x, y, width, height, color } => {
                    self.clear_rect(x, y, width, height, color);
                }
            }
        }

//         unsafe {
//             gl::DisableVertexAttribArray(0);
//             gl::DisableVertexAttribArray(1);
//             gl::BindVertexArray(0);

//             gl::Disable(gl::CULL_FACE);
//             gl::BindBuffer(gl::ARRAY_BUFFER, 0);
//             gl::BindTexture(gl::TEXTURE_2D, 0);
//         }

//         self.program.unbind();

//         self.check_error("render done");
    }

    fn create_image(&mut self, data: &DynamicImage, flags: ImageFlags) -> Result<Self::Image> {
        MtlTexture::new(data, flags)
    }

    fn update_image(&mut self, image: &mut Self::Image, data: &DynamicImage, x: usize, y: usize) -> Result<()> {
        image.update(data, x, y)
        
    }

    fn delete_image(&mut self, image: Self::Image) {
        image.delete();
    }

    fn screenshot(&mut self) -> Option<DynamicImage> {
        // todo!()
        let mut image = image::RgbaImage::new(self.view[0] as u32, self.view[1] as u32);

        // unsafe {
//             gl::ReadPixels(0, 0, self.view[0] as i32, self.view[1] as i32, gl::RGBA, gl::UNSIGNED_BYTE, image.deref_mut().as_ptr() as *mut GLvoid);
//         }

        image = image::imageops::flip_vertical(&image);

        Some(DynamicImage::ImageRgba8(image))
    }
}

// impl Drop for OpenGl {
//     fn drop(&mut self) {
//         if self.vert_arr != 0 {
//             unsafe { gl::DeleteVertexArrays(1, &self.vert_arr); }
//         }

//         if self.vert_buff != 0 {
//             unsafe { gl::DeleteBuffers(1, &self.vert_buff); }
//         }
//     }
// }
