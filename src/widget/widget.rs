use std::fmt::Debug;

use winit::event::WindowEvent;

pub mod macros {
    use crate::widget::Widget;
    pub fn is_widget<T: Widget>(widget: T) -> T {
        widget
    }

    #[macro_export]
    macro_rules! widget {
        (|$type:ty| $($field:ident : $value:expr),* $(,)?) => {
            {
                let mut instance: $type = Default::default();
                $(instance.$field = $value;)*
                instance
            }
        };
    }

    #[macro_export]
    macro_rules! widgets {
        [$($value:expr),* $(,)?] => {
            {
                let mut column: Widgets  = Vec::new();
                $(column.push(Box::new($value));)*
                column
            }
        };
    }

    pub(crate) use widget;
    pub(crate) use widgets;
}

macro_rules! common_widget {
    [] => {
        fn _get_mut_size(&mut self) -> (&mut Size, &mut Size) {
            (&mut self.width, &mut self.height)
        }
    }
}

// use crate::renderer::Renderer;

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Pixel(f32),
    Percent(f32),
}
pub type Widgets = Vec<Box<dyn Widget>>;

impl Default for Size {
    fn default() -> Self {
        Self::Pixel(0.0)
    }
}

pub trait Widget: std::fmt::Debug {
    fn _get_mut_size(&mut self) -> (&mut Size, &mut Size);
    fn get_size(&mut self) -> (Size, Size) {
        let (w, h) = self._get_mut_size();
        (w.clone(), h.clone())
    }
    fn set_size(&mut self, width: Size, height: Size) {
        let (w, h) = self._get_mut_size();
        *w = width;
        *h = height;
    }
    fn render(self);
}

#[derive(Debug)]
pub enum Layout {
    Flex(FlexLayout),
    Grid(GridLayout),
    Stack(StackLayout),
}

impl Default for Layout {
    fn default() -> Self {
        Layout::Stack(StackLayout {
            ..Default::default()
        })
    }
}

#[derive(Debug)]
pub struct Window {
    pub width: Size,
    pub height: Size,
    pub title: &'static str,
    pub child: Box<dyn Container>,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            width: Size::Pixel(0.0),
            height: Size::Pixel(0.0),
            title: "My Application",
            child: Box::new(FlexLayout {
                ..Default::default()
            }),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Label {
    pub width: Size,
    pub height: Size,
}

#[derive(Debug, Clone, Default)]
pub struct Button {
    pub width: Size,
    pub height: Size,
}

impl Widget for Label {
    common_widget!();
    fn render(self) {}
}
impl Widget for Button {
    common_widget!();
    fn render(self) {}
}
impl Widget for Window {
    common_widget!();
    fn render(self) {
        use crate::event::window::WindowEvent;
        use crate::platform::{Window, WindowInterface};
        let mut window = Window::start(self.title);
        let width = match self.width {
            Size::Pixel(number) => number,
            Size::Percent(number) => number,
        };
        let height = match self.width {
            Size::Pixel(number) => number,
            Size::Percent(number) => number,
        };

        window.set_size(width, height);

        window.handle_events(|event| match event {
            WindowEvent::WindowClosed => std::process::exit(0),
            _ => (),
        });

        window.show();
    }
}

pub trait Container: Widget {}

#[derive(Debug, Default)]
pub struct FlexLayout {
    pub width: Size,
    pub height: Size,
    pub children: Widgets,
}

#[derive(Debug, Clone, Default)]
pub struct GridLayout {
    pub width: Size,
    pub height: Size,
}

#[derive(Debug, Clone, Default)]
pub struct StackLayout {
    pub width: Size,
    pub height: Size,
}

impl Widget for FlexLayout {
    common_widget!();
    fn render(self) {}
}

impl Widget for GridLayout {
    common_widget!();
    fn render(self) {}
}

impl Widget for StackLayout {
    common_widget!();
    fn render(self) {}
}

impl Container for FlexLayout {}
impl Container for GridLayout {}
impl Container for StackLayout {}
