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

PROJECT IS ON PAUSE
