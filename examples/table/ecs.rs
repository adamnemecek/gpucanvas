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

pub struct Expanded;
pub struct Icon;

pub enum Event {

}

pub struct SysTableViewRenderer;

impl<'a> System<'a> for SysTableViewRenderer {
    type SystemData = (

    );
    fn run(&mut self, data: Self::SystemData) {
        todo!()
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