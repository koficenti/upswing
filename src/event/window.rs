#[derive(Clone)]
pub enum WindowEvent {
    WindowResize(f32, f32),
    WindowOpened,
    WindowClosed,
}
