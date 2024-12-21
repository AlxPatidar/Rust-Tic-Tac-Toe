mod app;
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = crate::app::App::default().run(&mut terminal);
    ratatui::restore();
    println!("Welcome to tick tock game:");
    app_result
}
