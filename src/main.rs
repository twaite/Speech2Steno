use iced::{button, Button, Column, Container, Element, Sandbox, Settings, Text};
use soundio::Context;

fn main() -> iced::Result {
    println!("Welcome to the Steno-S2T CLI");
    let mut ctx = Context::new();
    ctx.set_app_name("steno-test");
    ctx.connect().unwrap();
    ctx.flush_events();
    StenoS2T::run(Settings::default())
}

#[derive(Default)]
struct StenoS2T {
    //ctx: Context<'static>,
    button: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    RecordPressed,
}

impl Sandbox for StenoS2T {
    type Message = Message;

    fn new() -> Self {
        StenoS2T::default()
        //let mut ctx = Context::new();
        //ctx.set_app_name("steno-s2t");
        //ctx.connect();

        //let mut button = button::State::default();

        //Self { ctx, button }
    }

    fn title(&self) -> String {
        String::from("Steno - Speech to Text")
    }

    fn update(&mut self, message: Message) {
        match message {
            RecordPressed => (println!("Button pressed")),
        }
    }

    fn view(&mut self) -> Element<Message> {
        let title = Text::new("Steno S2T").size(24);

        let button = Button::new(&mut self.button, Text::new("● Record"))
            .padding(10)
            .on_press(Message::RecordPressed);

        let content = Column::new().push(title).push(button);

        Container::new(content).into()
    }
}
