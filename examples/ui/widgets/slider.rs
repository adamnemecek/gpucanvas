
use specs::prelude::*;
use gpucanvas::{
    Canvas,
    Paint,
    Color,
    renderer::OpenGl,
    Path,
    Rect,
};

pub struct Slider {
    data: f32,
    rect: Rect
}

pub trait UIRender<Atom> {
    fn draw(&mut self, atom: Atom);
}

impl UIRender<Slider> for Canvas<OpenGl> {
    fn draw(&mut self, atom: Slider) {
        let Rect { x, y, w, h } = atom.rect;
    //     NVGpaint bg, knob;
		let cy = y + (h * 0.5);
		let kr = (h * 0.25);

        self.save();

        // Slot
        let bg = Paint::box_gradient(
            x, cy - 2.0 + 1.0, w, 4.0,
            2.0, // radius
            2.0, // feather
            Color::rgba(0, 0, 0, 32), // inner_color: 
            Color::rgba(0, 0, 0, 128) // outer_color: 
        );

        // // // 	nvgRoundedRect(ctx, x, cy - 2, w, 4, 2);
        let path = Path::new();
        // path.rounded_rect(0.0, 0.0, 0.0, 0.0);

        // self.fill_paint(bg);

        // self.fill();

        // // Knob Shadow
        // let bg1 = Gradient::Radial{
        //     center: todo!(),
        //     in_radius: todo!(),
        //     inner_color: todo!(),
        //     out_radius: todo!(),
        //     outer_color: todo!(),
        // };
        // //	bg = nvgRadialGradient(ctx, x + (int)(pos*w), cy + 1, kr - 3, kr + 3, nvgRGBA(0, 0, 0, 64), nvgRGBA(0, 0, 0, 0));
        // self.begin_path();
        // //	nvgBeginPath(ctx);

        // //	nvgRect(ctx, x + (int)(pos*w) - kr - 5, cy - kr - 5, kr * 2 + 5 + 5, kr * 2 + 5 + 5 + 3);
        // //	nvgCircle(ctx, x + (int)(pos*w), cy, kr);
        // //	nvgPathWinding(ctx, NVG_HOLE);
        // //	nvgFillPaint(ctx, bg);
        // //	nvgFill(ctx);

        // // // Knob
        // // // 	knob = nvgLinearGradient(ctx, x, cy - kr, x, cy + kr, nvgRGBA(255, 255, 255, 16), nvgRGBA(0, 0, 0, 16));
        // let knob: Gradient = todo!();

        // // // 	nvgBeginPath(ctx);
        // self.begin_path();

        // // // 	nvgCircle(ctx, x + (int)(pos*w), cy, kr - 1);
        // // let circle = todo!();
        // // self.circle(circle);

        // // // 	nvgFillColor(ctx, nvgRGBA(40, 43, 48, 255));
        // let fill_color: crate::Color = todo!();
        // self.fill_color(fill_color);

        // self.fill();

        // self.fill_paint(knob);

        // self.fill();

        // self.begin_path();

        // // // 	nvgCircle(ctx, x + (int)(pos*w), cy, kr - 0.5f);
        // // self.circle(/*...*/);

        // // // 	nvgStrokeColor(ctx, nvgRGBA(0, 0, 0, 92));
        // // self.stroke_color(/*...*/);

        // self.stroke();

        // self.restore();
    }
}

impl Component for Slider {
    type Storage = DenseVecStorage<Self>;
}



pub struct SysSliderInput;

impl<'a> System<'a> for SysSliderInput {
    type SystemData = ();
    fn run(&mut self, data: Self::SystemData) {

    }
}
