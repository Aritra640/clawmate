// app state
pub struct App {
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            running: true,
        }
    }
}
