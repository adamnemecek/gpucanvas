use super::{Blend, Vertex, VertexOffsets};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct RPSKey {
    pub blend_func: Blend,
    pub pixel_format: metal::MTLPixelFormat,
}

// #[derive(Clone)]
pub struct RPS {
    pub id: usize,
    pub blend_func: Blend,
    pub pixel_format: metal::MTLPixelFormat,
    pub pipeline_state: metal::RenderPipelineState,
    pub stencil_only_pipeline_state: metal::RenderPipelineState,
    pub clear_rect_pipeline_state: metal::RenderPipelineState,
}

impl std::fmt::Debug for RPS {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_tuple("RPS")
            .field(&self.id)
            .field(&self.blend_func)
            .field(&self.pixel_format)
            .finish()
    }
}
impl PartialEq for RPS {
    fn eq(&self, other: &Self) -> bool {
        self.blend_func == other.blend_func && self.pixel_format == other.pixel_format
    }
}

impl RPS {
    fn new(
        id: usize,
        device: &metal::DeviceRef,
        blend_func: Blend,
        pixel_format: metal::MTLPixelFormat,
        vertex_descriptor: &metal::VertexDescriptorRef,
        vert_func: &metal::FunctionRef,
        frag_func: &metal::FunctionRef,
        clear_rect_vert_func: &metal::FunctionRef,
        clear_rect_frag_func: &metal::FunctionRef,
    ) -> Self {
        let desc = metal::RenderPipelineDescriptor::new();
        let color_attachment_desc = desc.color_attachments().object_at(0).unwrap();
        color_attachment_desc.set_pixel_format(pixel_format);

        desc.set_stencil_attachment_pixel_format(metal::MTLPixelFormat::Stencil8);
        desc.set_vertex_function(Some(vert_func));
        desc.set_fragment_function(Some(frag_func));
        desc.set_vertex_descriptor(Some(vertex_descriptor));
        desc.set_label(&format!(
            "rps: id: {:?}, blend_func: {:?}, pixel_format: {:?}",
            id, blend_func, pixel_format
        ));

        color_attachment_desc.set_blending_enabled(true);
        color_attachment_desc.set_source_rgb_blend_factor(blend_func.src_rgb);
        color_attachment_desc.set_source_alpha_blend_factor(blend_func.src_alpha);
        color_attachment_desc.set_destination_rgb_blend_factor(blend_func.dst_rgb);
        color_attachment_desc.set_destination_alpha_blend_factor(blend_func.dst_alpha);
        color_attachment_desc.set_write_mask(metal::MTLColorWriteMask::all());

        let pipeline_state = device.new_render_pipeline_state(&desc).unwrap();

        desc.set_fragment_function(None);
        desc.set_label(&format!(
            "stencil_only rps: blend_func: {:?}, pixel_format: {:?}",
            blend_func, pixel_format
        ));
        color_attachment_desc.set_write_mask(metal::MTLColorWriteMask::empty());
        let stencil_only_pipeline_state = device.new_render_pipeline_state(&desc).unwrap();

        // the rest of this function is not in metalnvg
        let clear_rect_pipeline_state = {
            let desc2 = metal::RenderPipelineDescriptor::new();
            let color_attachment_desc2 = desc2.color_attachments().object_at(0).unwrap();
            color_attachment_desc2.set_pixel_format(pixel_format);
            desc2.set_stencil_attachment_pixel_format(metal::MTLPixelFormat::Stencil8);
            desc2.set_fragment_function(Some(&clear_rect_frag_func));
            desc2.set_vertex_function(Some(&clear_rect_vert_func));

            desc2.set_label(&format!(
                "clear_rect rps: blend_func: {:?}, pixel_format: {:?}",
                blend_func, pixel_format
            ));

            color_attachment_desc2.set_blending_enabled(true);
            color_attachment_desc2.set_source_rgb_blend_factor(blend_func.src_rgb);
            color_attachment_desc2.set_source_alpha_blend_factor(blend_func.src_alpha);
            color_attachment_desc2.set_destination_rgb_blend_factor(blend_func.dst_rgb);
            color_attachment_desc2.set_destination_alpha_blend_factor(blend_func.dst_alpha);

            device.new_render_pipeline_state(&desc2).unwrap()
        };

        // clear_rect_pipeline_state.set_label("clear_rect_pipeline_state");
        Self {
            id,
            pipeline_state,
            blend_func,
            pixel_format,
            clear_rect_pipeline_state,
            stencil_only_pipeline_state,
        }
    }
}

use std::rc::Rc;
pub struct RPSCache {
    pub device: metal::Device,
    vertex_descriptor: metal::VertexDescriptor,
    vert_func: metal::Function,
    frag_func: metal::Function,
    clear_rect_vert_func: metal::Function,
    clear_rect_frag_func: metal::Function,

    inner: HashMap<RPSKey, Rc<RPS>>,
    id: usize,
}

impl RPSCache {
    pub fn new(device: &metal::DeviceRef, library: &metal::LibraryRef, antialias: bool) -> Self {
        let vert_func = library
            .get_function("vertexShader", None)
            .expect("vert shader not found");

        let frag_func: metal::Function = if antialias {
            library
                .get_function("fragmentShaderAA", None)
                .expect("frag shader not found")
        } else {
            library
                .get_function("fragmentShader", None)
                .expect("frag shader not found")
        };

        let clear_rect_vert_func = library
            .get_function("clear_rect_vertex", None)
            .expect("clear_rect_vertex shader not found");

        let clear_rect_frag_func = library
            .get_function("clear_rect_fragment", None)
            .expect("clear_rect_fragment shader not found");

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

        Self {
            device: device.to_owned(),
            vertex_descriptor: vertex_descriptor.to_owned(),
            vert_func,
            frag_func,
            clear_rect_vert_func,
            clear_rect_frag_func,
            inner: Default::default(),
            id: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn get(&mut self, blend_func: Blend, pixel_format: metal::MTLPixelFormat) -> Rc<RPS> {
        let key = RPSKey {
            blend_func,
            pixel_format,
        };
        if !self.inner.contains_key(&key) {
            let rps = RPS::new(
                self.id,
                &self.device,
                blend_func,
                pixel_format,
                &self.vertex_descriptor,
                &self.vert_func,
                &self.frag_func,
                &self.clear_rect_vert_func,
                &self.clear_rect_frag_func,
            );
            self.id += 1;

            self.inner.insert(key, Rc::new(rps));
        }
        // clone adds a reference
        self.inner.get(&key).unwrap().clone()
    }
}
