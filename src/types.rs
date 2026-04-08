#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Overview,
    Cpu,
}

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Page),
}
