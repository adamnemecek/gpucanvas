use specs::prelude::*;
use specs_hierarchy::{Hierarchy, Parent as HParent};
use gpucanvas::Canvas;
use gpucanvas::renderer::OpenGl;


pub struct Slider {
    data: f32
}

impl Component for Slider {
    type Storage = DenseVecStorage<Self>;
}

struct SysSliderInput;

impl<'a> System<'a> for SysSliderInput {
    type SystemData = ();
    fn run(&mut self, data: Self::SystemData) {

    }
}


/// Component for defining a parent entity.
///
/// The entity with this component *has* a parent, rather than *is* a parent.
///
///
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Parent {
    /// The parent entity
    pub entity: Entity,
}

impl Component for Parent {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl HParent for Parent {
    fn parent_entity(&self) -> Entity {
        self.entity
    }
}

pub struct Rect {

}

pub struct IndexPath {
    data: Vec<usize>
}

impl Component for IndexPath {
    type Storage = DenseVecStorage<Self>;
}

pub enum Status {
    Collapsed, Expanded
}
impl Default for Status {
    fn default() -> Self {
        Self::Collapsed
    }
}

impl Component for Status {
    type Storage = DenseVecStorage<Self>;
}


#[derive(Default)]
pub struct ItemStatus {
    data: Status
}

// these are cached for the view that are currently visible
pub struct Texture {
    // data: metal::Texture
}

pub struct SysMouseInput;

impl<'a> System<'a> for SysMouseInput {
    type SystemData = (

    );

    fn run(&mut self, data: Self::SystemData) {

    }
}

// impl Component for ItemStatus {
//     type Storage = NullStorage<Self>;
// }

pub struct Label {
    data: String
}

// impl std::default::Default for Expanded {
//     fn default() -> Self {
//         Self {}
//     }
// }

fn draw_dropdown(canvas: &mut Canvas<OpenGl>, text: &str, rect: Rect, status: Status) {
    // void DrawDropDown(DrawContext* context, const char* text, float x, float y, float w, float h)
	// {
	// 	NVGpaint bg;
	// 	char icon[8];
	// 	float cornerRadius = 2.0f;

	// 	bg = nvgLinearGradient(ctx, x, y, x, y + h, nvgRGBA(255, 255, 255, 16), nvgRGBA(0, 0, 0, 16));
	// 	nvgBeginPath(ctx);
	// 	nvgRoundedRect(ctx, x + 1, y + 1, w - 2, h - 2, cornerRadius - 1);
	// 	nvgFillPaint(ctx, bg);
	// 	nvgFill(ctx);

	// 	nvgBeginPath(ctx);
	// 	nvgRoundedRect(ctx, x + 0.5f, y + 0.5f, w - 1, h - 1, cornerRadius - 0.5f);
	// 	nvgStrokeColor(ctx, nvgRGBA(0, 0, 0, 48));
	// 	nvgStroke(ctx);

	// 	nvgFontSize(ctx, theme->standardFontSize);
	// 	nvgFontFace(ctx, "sans");
	// 	nvgFillColor(ctx, theme->textColor);
	// 	nvgTextAlign(ctx, NVG_ALIGN_LEFT | NVG_ALIGN_MIDDLE);
	// 	nvgText(ctx, x + h * 0.3f, y + h * 0.5f, text, NULL);

	// 	nvgFontSize(ctx, h);
	// 	nvgFontFace(ctx, "ui");
	// 	nvgFillColor(ctx, theme->textColor);
	// 	nvgTextAlign(ctx, NVG_ALIGN_CENTER | NVG_ALIGN_MIDDLE);
	// 	nvgText(ctx, x + w - h * 0.5f, y + h * 0.5f, cpToUTF8(ICON_UNFOLD_MORE, icon), NULL);
	// }
}

pub struct Icon {

}

pub enum Event {

}

pub struct EventTableViewMouse {

}

pub struct SysTableViewInput {
    reader_id: ReaderId<EventTableViewMouse>
}

impl<'a> System<'a> for SysTableViewInput {
    type SystemData = (
        // rect
        // selected
        //
    );
    fn run(&mut self, data: Self::SystemData) {
        // find the entry that was hit and make it selected
        // if the hit was on the icon, expand it
        // if shift was down, don't
    }
}

// layout system orders the items by their indexpath and
pub struct SysTableViewLayout {

}

impl SysTableViewLayout {
    fn run() {
        /*
        let offset = 0;
        for entity, rect, status, size in data {
            if status == Status::Collapsed {
                offset += size.h;
            }
        }
        */

    }
}

pub struct SysTableViewRenderer;

impl<'a> System<'a> for SysTableViewRenderer {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Status>
        // IndexPath
        // Icon
        // Label
        // Rect
        // status
    );

    fn run(&mut self, data: Self::SystemData) {
        // sort by indexpath (eventually, the entities will be sorted by their index path)
        // iterate through them and render into canvas
        /*
        for (ent, status) in data.join() {
            let entry = todo!();
            if status == Status::Collapsed {
                continue;
            }
            // draw_entry(canvas, label)
        }
        */
    }
}


pub struct App {
    world: World,
    canvas: Canvas<OpenGl>,
    // dispatcher
}

impl App {
    pub fn new(canvas: Canvas<OpenGl>) -> Self {
        // Self { canvas }
        // let a =
        todo!()
    }

    pub fn input(&mut self, event: Event) {
        // hierarchy
    }


}