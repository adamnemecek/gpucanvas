use specs::prelude::*;
use specs_hierarchy::{Hierarchy, Parent as HParent};
use gpucanvas::Canvas;
use gpucanvas::renderer::OpenGl;

/// Component for defining a parent entity.
///
/// The entity with this component *has* a parent, rather than *is* a parent.
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

enum Status {
    Collapsed, Expanded
}
impl Default for Status {
    fn default() -> Self {
        Self::Collapsed
    }
}

#[derive(Default)]
pub struct ItemStatus {
    data: Status
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

fn draw_dropdown(canvas: &mut Canvas<OpenGl>, text: &str) {
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

pub struct SysTableViewInput {
    reader_id: ReaderId<ComponentEvent>
}

impl<'a> System<'a> for SysTableViewInput {
    type SystemData = (

    );
    fn run(&mut self, data: Self::SystemData) {

    }
}
pub struct SysTableViewRenderer;

impl<'a> System<'a> for SysTableViewRenderer {
    type SystemData = (
        // IndexPath
        // Icon
        // Label
        //
    );

    fn run(&mut self, data: Self::SystemData) {
        // sort by indexpath
        // iterate through them and render into
        // for e in data.join() {
            // let entry =
            // draw_entry(canvas, )
        // }
    }
}


pub struct App {
    canvas: Canvas<OpenGl>
}

impl App {
    pub fn new(canvas: Canvas<OpenGl>) -> Self {
        Self { canvas }
    }

    pub fn input(&mut self, event: Event) {

    }
}