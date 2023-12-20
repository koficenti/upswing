use std::sync::{Arc, Mutex};

use crate::event::{window::WindowEvent, Event};
use winit::{
    event_loop,
    platform::{self, windows::EventLoopBuilderExtWindows},
    window,
};

pub trait WindowInterface {
    fn show(self);
    fn hide(&self);
    fn set_title(&self);
    fn set_size(&mut self, width: f32, height: f32);
    fn get_size(&self);
    fn set_pos(&self);
    fn get_pos(&self);
    fn set_resizeable(&self);
    fn is_resizeable(&self);
    fn set_fullscreen(&self);
    fn handle_events<'a, F>(&self, fun: F)
    where
        F: Fn(WindowEvent) + 'a + 'static;
    fn close(&self);
}

pub struct WinitWindow {
    width: f32,
    height: f32,
    posx: f32,
    posy: f32,
    resizeable: bool,
    fullscreen: bool,

    events: Arc<Mutex<Event<WindowEvent>>>,
    _winit: winit::window::Window,
    _winit_eventloop: event_loop::EventLoop<()>,
}

impl WindowInterface for WinitWindow {
    fn show(self) {
        self._winit_eventloop.run(move |event, _, _| match event {
            winit::event::Event::WindowEvent { window_id, event } => {
                let custom_event = self.events.lock().unwrap();
                match event {
                    winit::event::WindowEvent::CloseRequested => {
                        custom_event.trigger(WindowEvent::WindowClosed)
                    }
                    winit::event::WindowEvent::Resized(size) => {
                        custom_event.trigger(WindowEvent::WindowResize(
                            size.width as f32,
                            size.height as f32,
                        ));
                    }
                    _ => (),
                }
            }
            _ => (),
        })
    }
    fn hide(&self) {}
    fn set_title(&self) {}
    fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self._winit
            .set_inner_size(winit::dpi::LogicalSize::<f32>::new(width, height));
    }
    fn get_size(&self) {}
    fn set_pos(&self) {}
    fn get_pos(&self) {}
    fn set_resizeable(&self) {}
    fn is_resizeable(&self) {}
    fn set_fullscreen(&self) {}
    fn handle_events<'a, F>(&self, fun: F)
    where
        F: Fn(WindowEvent) + 'a + 'static,
    {
        self.events.lock().unwrap().subscribe(fun);
    }
    fn close(&self) {
        self.events
            .lock()
            .unwrap()
            .trigger(WindowEvent::WindowClosed);
    }
}
pub struct Window {}

impl Window {
    pub fn start(title: &str) -> impl WindowInterface {
        #[cfg(test)]
        let _winit_eventloop = event_loop::EventLoop::from(
            event_loop::EventLoopBuilder::new()
                .with_any_thread(true)
                .build(),
        );
        #[cfg(not(test))]
        let _winit_eventloop = event_loop::EventLoop::new();

        let _winit = winit::window::WindowBuilder::new()
            .with_title(title)
            .build(&_winit_eventloop)
            .expect("Window could not be created");
        let window = WinitWindow {
            width: 0.,
            height: 0.,
            posx: 0.,
            posy: 0.,
            resizeable: true,
            fullscreen: false,
            events: Arc::new(Mutex::new(Event::new())),
            _winit,
            _winit_eventloop,
        };
        return window;
    }
}
