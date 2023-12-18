#[cfg(test)]


pub mod tests {
    use crate::{window::run, event::Event};
    
    #[test]
    fn events(){
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
    fn create_window(){
        pollster::block_on(run("my app"));
        assert!(true);
    }
}