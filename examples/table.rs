use std::time::Instant;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

use gpucanvas::{
    Renderer,
    Canvas,
    Color,
    Paint,
    Align,
    Baseline,
    Path,
    ImageId,
    ImageFlags,
    Weight,
    renderer::OpenGl
};

fn main() {

    let window_size = glutin::dpi::PhysicalSize::new(1000, 600);
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_inner_size(window_size)
        .with_resizable(false)
        .with_title("Text demo");

    let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    let renderer = OpenGl::new(|s| windowed_context.get_proc_address(s) as *const _).expect("Cannot create renderer");
    let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");
    canvas.set_size(window_size.width as u32, window_size.height as u32, windowed_context.window().scale_factor() as f32);

    let _ = canvas.add_font("examples/assets/NotoSans-Regular.ttf");
    let _ = canvas.add_font("examples/assets/Roboto-Regular.ttf");
    let _ = canvas.add_font("examples/assets/Roboto-Bold.ttf");
    let _ = canvas.add_font("examples/assets/Roboto-Light.ttf");
    let _ = canvas.add_font("examples/assets/amiri-regular.ttf");

    let flags = ImageFlags::GENERATE_MIPMAPS | ImageFlags::REPEAT_X | ImageFlags::REPEAT_Y;
    let image_id = canvas.create_image_file("examples/assets/pattern.jpg", flags).expect("Cannot create image");

    let start = Instant::now();
    let mut prevt = start;

    // let mut perf = PerfGraph::new();

    let mut font_size = 18;

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(*physical_size);
                }
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                }
                WindowEvent::MouseWheel { device_id: _, delta, .. } => match delta {
                    glutin::event::MouseScrollDelta::LineDelta(_, y) => {
                        font_size += *y as i32;
                        font_size = font_size.max(2);
                    },
                    _ => ()
                }
                _ => (),
            }
            Event::RedrawRequested(_) => {
                let dpi_factor = windowed_context.window().scale_factor();
                let size = windowed_context.window().inner_size();
                canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
                canvas.clear_rect(0, 0, size.width as u32, size.height as u32, Color::rgbf(0.9, 0.9, 0.9));

                let elapsed = start.elapsed().as_secs_f32();
                let now = Instant::now();
                let dt = (now - prevt).as_secs_f32();
                prevt = now;

                draw_metalnanovg_test(&mut canvas, 10.0, 10.0, size.width as f32, size.height as f32);

                canvas.save();
                canvas.reset();
                // perf.render(&mut canvas, 5.0, 5.0);
                canvas.restore();

                canvas.flush();
                windowed_context.swap_buffers().unwrap();
            }
            Event::MainEventsCleared => {
                windowed_context.window().request_redraw()
            }
            _ => (),
        }
    });
}
// pub fn box_gradient(x: f32, y: f32, width: f32, height: f32, radius: f32, feather: f32, inner_color: Color, outer_color: Color) -> Self {

fn draw_metalnanovg_test<T: Renderer>(canvas: &mut Canvas<T>, x: f32, y: f32, w: f32, h: f32) {
    // mnvgClearWithColor(ctx, nvgRGBA(255,128,0,255));
    let clear_color = Color::rgba(255,128,0,255);
    canvas.clear_rect(0, 0, w as u32, h as u32, clear_color);
    // bg = nvgBoxGradient(ctx, x,y+1.5f, w,h, h/2,5, nvgRGBA(0,0,0,16), nvgRGBA(0,0,0,92));
    let inner_color = Color::rgba(0,0,0,16);
    let outer_color = Color::rgba(0,0,0,92);
    let radius: f32 = h/2.0;
    let feather: f32 = 5.0;
    let bg = Paint::box_gradient(x, y, w, h, radius, feather, inner_color, outer_color);
    // nvgBeginPath(ctx);
    // nvgRoundedRect(ctx, x,y, w,h, cornerRadius);
    // nvgFillPaint(ctx, bg);
    // nvgFill(ctx);

    let mut path = Path::new();
    path.rounded_rect(
        x,
        y,
        w,
        h,
        10.0,
    );

    canvas.fill_path(&mut path, bg);
}