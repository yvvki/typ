use iced::{
    button, Alignment, Button, Column, Element, Sandbox, Text,
};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Typ {
    count: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

impl Sandbox for Typ {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Typ")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::Increment),
            )
            .push(Text::new(self.count.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::Decrement),
            )
            .into()
    }
}
