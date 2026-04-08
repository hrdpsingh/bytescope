mod pages;
mod styles;
mod types;
mod ui;

use iced::{Application, Settings};
use ui::Probe;

fn main() -> iced::Result {
    Probe::run(Settings::default())
}
