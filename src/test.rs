#[cfg(test)]

pub mod tests {
    use std::process::exit;

    use crate::event::window::WindowEvent;

    use crate::platform::{Window, WindowInterface};
    use crate::widget::macros::*;
    use crate::widget::*;
    use crate::{event::Event, window::run};

    #[test]
    fn events() {
        let mut event = Event::<String>::new();
        event.subscribe(|event| {
            println!("say once: {}", event);
        });

        event.subscribe(|event| {
            println!("say twice: {}", event);
        });

        event.trigger("Gonna be repeated two times total.".to_string());
    }

    #[test]
    fn create_window() {
        // let mut window = Window::start("hello world");

        // window.set_size(200.0, 300.0);

        // window.handle_events(|event| {
        //     match event {
        //         WindowEvent::WindowClosed => exit(0),
        //         WindowEvent::WindowResize(x, y) => println!("Size({},{})", x, y),
        //         _ => ()
        //     }
        // });

        // window.show();
    }

    #[test]
    fn widgets_test() {
        use Size::*;

        widget!( | widget::Window |
            
            title:  "My Super Cool App",
            width:  Pixel(500.0),
            height: Pixel(500.0),

            child: Box::new (
                widget!( | widget::FlexLayout |
                    width: Pixel(10.0),
                    height: Pixel(10.0),

                    children: widgets![
                        widget!( | Button |
                            width: Pixel(100.0),
                            height: Pixel(100.0)
                        )
                    ],
                ),
            ),
        )
        .render();
    }
}
