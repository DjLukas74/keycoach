use iced::{button, Button, Column, Element, Sandbox, Settings, Text, window::Position};

#[derive(Default)]
pub struct MyApp {
    click_count: u32,
    click_button: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Clicked,
}

impl Sandbox for MyApp {
    type Message = Message;

    fn new() -> Self {
        MyApp::default()
    }

    fn title(&self) -> String {
        String::from("My Rust GUI App")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Clicked => {
                self.click_count += 1;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .push(Text::new("Welcome to My Rust GUI App!"))
            .push(
                Button::new(&mut self.click_button, Text::new("Click Me!"))
                    .on_press(Message::Clicked),
            )
            .push(Text::new(format!("Click count: {}", self.click_count)))
            .into()
    }
}
