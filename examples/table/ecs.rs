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



pub struct IndexPath {
    data: Vec<usize>
}

impl Component for IndexPath {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Expanded;
impl Component for Expanded {
    type Storage = NullStorage<Self>;
}

// impl std::default::Default for Expanded {
//     fn default() -> Self {
//         Self {}
//     }
// }

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
        //
    );

    fn run(&mut self, data: Self::SystemData) {

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