use super::Blend;
use std::hash::{Hash, Hasher};

pub struct RPS {
    // pub device: metal::Device,
    pub blend_func: Blend,
    pub pipeline_state: metal::RenderPipelineState,
    pub stencil_only_pipeline_state: metal::RenderPipelineState,
    pub pipeline_pixel_format: metal::MTLPixelFormat,
    pub clear_rect_pipeline_state: metal::RenderPipelineState,
}

impl RPS {
    fn new(
        device: &metal::DeviceRef,
        blend_func: Blend,
        pipeline_pixel_format: metal::MTLPixelFormat,
        vertex_descriptor: &metal::VertexDescriptorRef,
        vert_func: &metal::FunctionRef,
        frag_func: &metal::FunctionRef,
        clear_rect_vert_func: &metal::FunctionRef,
        clear_rect_frag_func: &metal::FunctionRef,
    ) -> Self {
        let desc = metal::RenderPipelineDescriptor::new();
        let color_attachment_desc = desc.color_attachments().object_at(0).unwrap();
        color_attachment_desc.set_pixel_format(pipeline_pixel_format);

        // println!("blend: {:?}", blend_func);
        desc.set_stencil_attachment_pixel_format(metal::MTLPixelFormat::Stencil8);
        desc.set_vertex_function(Some(vert_func));
        desc.set_fragment_function(Some(frag_func));
        desc.set_vertex_descriptor(Some(vertex_descriptor));

        color_attachment_desc.set_blending_enabled(true);
        color_attachment_desc.set_source_rgb_blend_factor(blend_func.src_rgb);
        color_attachment_desc.set_source_alpha_blend_factor(blend_func.src_alpha);
        color_attachment_desc.set_destination_rgb_blend_factor(blend_func.dst_rgb);
        color_attachment_desc.set_destination_alpha_blend_factor(blend_func.dst_alpha);

        // self.blend_func = blend_func;
        let pipeline_state = device.new_render_pipeline_state(&desc).unwrap();
        // pipeline_state.set_label("pipeline_state");

        desc.set_fragment_function(None);
        color_attachment_desc.set_write_mask(metal::MTLColorWriteMask::empty());
        let stencil_only_pipeline_state = device.new_render_pipeline_state(&desc).unwrap();
        // stencil_only_pipeline_state.set_label("stencil_only_pipeline_state");

        // the rest of this function is not in metalnvg
        let clear_rect_pipeline_state = {
            let desc2 = metal::RenderPipelineDescriptor::new();
            let color_attachment_desc2 = desc2.color_attachments().object_at(0).unwrap();
            color_attachment_desc2.set_pixel_format(pipeline_pixel_format);
            // color_attachent_desc.set_pixel_format(metal::MTLPixelFormat::BGRA8Unorm);;
            desc2.set_stencil_attachment_pixel_format(metal::MTLPixelFormat::Stencil8);
            desc2.set_fragment_function(Some(&clear_rect_frag_func));
            desc2.set_vertex_function(Some(&clear_rect_vert_func));

            color_attachment_desc2.set_blending_enabled(true);
            color_attachment_desc2.set_source_rgb_blend_factor(blend_func.src_rgb);
            color_attachment_desc2.set_source_alpha_blend_factor(blend_func.src_alpha);
            color_attachment_desc2.set_destination_rgb_blend_factor(blend_func.dst_rgb);
            color_attachment_desc2.set_destination_alpha_blend_factor(blend_func.dst_alpha);

            device.new_render_pipeline_state(&desc2).unwrap()
        };

        // clear_rect_pipeline_state.set_label("clear_rect_pipeline_state");
        Self {
            pipeline_state,
            blend_func,
            pipeline_pixel_format,
            clear_rect_pipeline_state,
            stencil_only_pipeline_state,
        }
    }
}

impl Hash for RPS {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}

pub struct RPSCache {
    pub device: metal::Device,
    vertex_descriptor: metal::VertexDescriptor,
    vert_func: metal::Function,
    frag_func: metal::Function,
    clear_rect_vert_func: metal::Function,
    clear_rect_frag_func: metal::Function,

}

impl RPSCache {
    pub fn new(device: &metal::DeviceRef) -> Self {
        todo!()
    }

    pub fn get(&mut self, pipeline_pixel_format: metal::MTLPixelFormat, blend_func: Blend) -> RPS {
        todo!()
    }
}
