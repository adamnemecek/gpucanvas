pub fn generate_mipmaps(command_queue: &metal::CommandQueueRef, tex: &metal::TextureRef) {
    let command_buffer = command_queue.new_command_buffer();
    let encoder = command_buffer.new_blit_command_encoder();
    encoder.generate_mipmaps(&tex);

    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();
}


pub trait RenderCommandEncoderExt {
    fn set_vertex_value<T>(&self, index: u64, value: &T);
}

impl RenderCommandEncoderExt for metal::RenderCommandEncoderRef {
    fn set_vertex_value<T>(&self, index: u64, value: &T) {
        let ptr = value as *const T;
        self.set_vertex_bytes(
            index,
            std::mem::size_of::<T>() as u64,
            ptr as *const _
        )
    }
}