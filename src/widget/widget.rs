use crate::renderer::Renderer;

pub trait Widget {
    fn new(&self);
    fn build(&self);

    fn render(&self, renderer: impl Renderer);
    fn events(&self);
    fn update(&self);
}