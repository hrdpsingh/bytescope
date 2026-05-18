mod app;
mod components;
mod models;
mod pages;
use app::Probe;

fn main() -> iced::Result {
    iced::application(Probe::default, Probe::update, Probe::view)
        .subscription(Probe::subscription)
        .title("Probe")
        .run()
}
