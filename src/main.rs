use iced::widget::{row, text};
use iced::{Alignment, Application, Command, Element, Settings, Theme, executor};

fn main() -> iced::Result {
    ByteScope::run(Settings::default())
}

struct ByteScope {
    hostname: String,
}

#[derive(Debug, Clone)]
enum Message {}

impl Application for ByteScope {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let hostname = match hostname::get() {
            Ok(name) => name.to_string_lossy().to_string(),
            Err(_) => "Unavailable".to_string(),
        };

        (Self { hostname }, Command::none())
    }

    fn title(&self) -> String {
        String::from("ByteScope")
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        row![text("Hostname: "), text(&self.hostname)]
            .padding(20)
            .align_items(Alignment::Center)
            .into()
    }
}
