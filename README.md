# Work In Progress

```rust
    // From src/test.rs
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
        ).render();
    }

```

Basic widget structure has been setup. Still needs a ton of work but at least I came up with the structure 😋

Next I'm gonna work on the layout calculations, oh and add a draw_rect function. So I can visualize it.

So far so good I think.