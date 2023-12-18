// Simple placeholder
pub trait Renderer {
    fn render_text(&self, text: &str);
    fn render_image(&self, path: &str);
    fn render_button(&self, label: &str);
}